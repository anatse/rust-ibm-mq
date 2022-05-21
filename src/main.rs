mod mq;
mod kafka;
mod model;
mod config;

use log::{debug, info, error, log_enabled};
use log::Level;
use std::{env, thread};
use std::process::exit;
use std::sync::Arc;

use mq::ibm_mq::IbmMqFactory;
use crate::config::config::Config;

use actix_web::{get, post, middleware, web, App, HttpRequest, HttpServer, Responder};
use crate::kafka::producer::KafkaProducer;
use crate::model::bases::{BaseResponse, ErrorMessage};
use crate::model::event::{AuditEventBuilder, AuditEvent};
use rdkafka::producer::DeliveryFuture;
use crate::model::metamodel::MetaModel;
use uuid::Uuid;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use crate::mq::ibm_mq::MqMessage;

type WebData = (Arc<KafkaProducer>, Arc<IbmMqFactory>, Arc<AtomicBool>, Arc<AtomicBool>);

/// Process kafka result and convert it to BaseResponse
/// # Argument
/// * `future` - future for kafka operation result
async fn process_result(df: DeliveryFuture) -> BaseResponse<String> {
    match df.await {
        Ok(e) => {
            if log_enabled!(Level::Debug) {
                debug!("Send was successful? {}", e.is_err());
            }
            if e.is_err() {
                BaseResponse::error(
                    ErrorMessage::new(
                        e.err().map(|err|
                            format!("{}/{:?}", err.0.to_string(), err.1))
                            .expect("Error getting error")
                    )
                )
            } else {
                BaseResponse::success(e.ok().map(|res|
                    format!("{}/{}", res.0, res.1))
                    .expect("Error getting result"))
            }
        },
        Err(e) => {
            error!("Error occurred: {:?}", e);
            BaseResponse::error(ErrorMessage::new("Unknown error".to_string()))
        },
    }
}

/// Function process kafka result
/// # Argument
/// * `future` - future for kafka operation result
async fn process_kafka_result(future: DeliveryFuture) -> bool {
    match future.await {
        Ok(v) => {
            match v {
                Ok(res) => {
                    if log_enabled!(Level::Info) {
                        info!("Send event to kafka successfully {:?}", res);
                    }
                    true
                },
                Err(e) => {
                    error!("[{:?}] Error sending event to kafka {:?}", thread::current().id(), e);
                    false
                },
            }
        },
        Err(e) => {
            error!("Error sending event to kafka {:?}", e);
            false
        }
    }
}

#[get("/mq/stop")]
async fn stop_listeners(data: web::Data<WebData>) -> impl Responder {
    let (_, _, running, _) = data.get_ref();
    running.store(false, Ordering::Relaxed);
    let response = BaseResponse::success("event");
    web::Json(response)
}

#[get("/mq/start")]
async fn start_listeners(data: web::Data<WebData>) -> impl Responder {
    let (_, _, running, _) = data.get_ref();
    running.store(true, Ordering::Relaxed);
    let response = BaseResponse::success("event");
    web::Json(response)
}

#[get("/mq/send_test")]
async fn send_test(data: web::Data<WebData>) -> impl Responder {
    let (_, send_factory, _, _) = data.get_ref();
    debug!("Start sending test messages...");

    for index in 0..5 {
        let event = AuditEventBuilder::new()
            .uuid("135676".to_string())
            .subsystem_code("AUDIT".to_string())
            .event_code("TEST_EVENT".to_string())
            .group_code("GROUP".to_string())
            .sector(Some("LOG".to_string()))
            .channel(Some("WEB".to_string()))
            .request_id(Some("request_id".to_string()))
            .audit_context_uuid(Some(Uuid::new_v4().to_string()))
            .meta_model_version("test-meta-model".to_string())
            .event_date_time(100000)
            .success(true)
            .init_context(Some(true))
            .sector(Some(format!("Index: {}", index)))
            .build();

        let json = serde_json::to_string(&event).unwrap();
        send_factory.send_message_with_props(
            json.clone(),
            &([
                ("Criticality".to_string(), "CRITICAL".to_string()),
                ("MessageType".to_string(), "auditEvent".to_string()),
                ("ItemType".to_string(), "SingleItem".to_string()),
            ].iter().cloned().collect())
        ).unwrap();
    }

    let response = BaseResponse::success("event");
    web::Json(response)
}

#[inline(always)]
fn is_critical(crit_opt: &Option<String>) -> bool {
    if let Some(criticality) = crit_opt {
        "CRITICAL" == criticality.to_uppercase()
    } else {
        true
    }
}

/// REST сервис для отправки события аудита в кафку
/// # Аргументы
/// * `_req` - HTTP запрос
/// * `event`- событие аудита для отправки
/// Возвращает успешность операции
#[post("/event/send")]
async fn event_send(data: web::Data<WebData>, _req: HttpRequest, event: web::Json<AuditEvent>) -> impl Responder {
    let (kp, _, _, _) = data.get_ref();
    let av = event.0;
    let res = kp.send_event(&av, is_critical(&av.criticality));
    web::Json(process_result(res).await)
}

#[post("/batch/send")]
async fn batch_send(data: web::Data<WebData>, _req: HttpRequest, event: web::Json<Vec<AuditEvent>>) -> impl Responder {
    let (kp, _, _, _) = data.get_ref();
    let res = kp.send_batch_event(&event.0);
    web::Json(process_result(res).await)
}

#[post("/mm/send")]
async fn meta_model_send(data: web::Data<WebData>, _req: HttpRequest, meta_model: web::Json<MetaModel>) -> impl Responder {
    let (kp, _, _, _) = data.get_ref();
    let res = kp.send_meta_model(&meta_model.0);
    web::Json(process_result(res).await)
}

fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(stop_listeners);
    cfg.service(start_listeners);
    cfg.service(send_test);
    cfg.service(event_send);
    cfg.service(batch_send);
    cfg.service(meta_model_send);
}

/// Function send messages to Kafka depends on MQ message headers
/// # Arguments
/// * `producer` - Kafka producer to send messages
/// * `message` - MQ message
async fn send_to_kafka(producer: Arc<KafkaProducer>, message: MqMessage) -> bool {
    if log_enabled!(Level::Debug) {
        debug!("Thread: {:?}", thread::current().id());
    }

    if let Some(ev_type) = message.headers.get("MessageType") {
        if log_enabled!(Level::Debug) {
            debug!("Found MessageType: {}", ev_type);
        }
        match ev_type.as_str() {
            "auditEvent" => {
                debug!("Sending audit event");
                // We are need to parse event to know level of criticality
                let critical = if let Ok(event) = serde_json::from_str::<AuditEvent>(message.body.as_str()) {
                    if log_enabled!(Level::Debug) {
                        debug!("Message parsed, criticality: {:?}", &event.criticality);
                    }
                    is_critical(&event.criticality)
                } else {
                    false
                };

                process_kafka_result(producer.send_event_raw(message.body.as_str(), critical)).await
            },
            "metaModel" => {
                debug!("Sending metaModel");
                process_kafka_result(producer.send_meta_model_raw(message.body.as_str())).await
            },
            "batchAuditEvent" => {
                debug!("Sending eventBatch");
                process_kafka_result(producer.send_batch_event_raw(message.body.as_str())).await
            },
            header => {
                if log_enabled!(Level::Debug) {
                    debug!("Unknown header: '{}'", header);
                }
                false
            }
        }
    } else {
        if log_enabled!(Level::Debug) {
            debug!("Not found MessageType header: '{}'", message.headers["MessageType"]);
        }
        false
    }
}

/// Function reads message
/// # Arguments
/// * `factory` - Ibm MQ factory to receive messages
/// * `producer` - Kafka producer to send messages
/// * `running` - atomic flag to manage message getting loop
/// * `starting` - atomic flag to manage fully start/stop loop
async fn read_messages(factory: &IbmMqFactory,
                       producer: Arc<KafkaProducer>,
                       running: Arc<AtomicBool>,
                       starting : Arc<AtomicBool>) {
    debug!("Start reading messages");

    while starting.load(Ordering::Relaxed) {
        while running.load(Ordering::Relaxed) {
            match factory.get_message(|message| {
                let prod = producer.to_owned();
                async move {
                    if log_enabled!(Level::Debug) {
                        debug!("Income message: {:?}", message);
                    }
                    send_to_kafka(prod.to_owned(), message.to_owned()).await
                }
            }).await {
                Ok(msg) => {
                    if log_enabled!(Level::Debug) {
                        debug!("Get message {:?}", msg);
                    }
                },
                Err(err) => {
                    if err.cReason != 2033 {
                        error!("Error getting message: {:?}", err);
                    }
                    tokio::time::delay_for(Duration::from_millis(10)).await;
                }
            }
        }

        debug!("Finish reading messages");
        // Starting checked only for every 5 seconds
        tokio::time::delay_for(Duration::from_secs(1)).await;
    }
}

// #[tokio::main(core_threads = 32)]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: audit-rest <config>");
        exit(-1);
    }

    debug!("Starting ...");

    let config = Arc::new(Config::load(args[1].to_owned().as_str()));

    let keep_alive = config.to_owned().http.keep_alive;
    let shutdown_timeout = config.http.shutdown_timeout.to_owned();
    let bind_url = format!("{}:{}", config.http.host, config.http.port);

    let running = Arc::new(AtomicBool::new(true));
    let starting = Arc::new(AtomicBool::new(true));

    let producer = Arc::new(KafkaProducer::new(config.to_owned()));

    // let rt = tokio::runtime::Runtime::new().unwrap();
    // let _ = actix_rt::System::run_in_tokio("test", &tokio::task::LocalSet::new());

    config.mq.factories.iter().filter(move |mq| mq.enabled).for_each( |mq| {
        for _ in 0..config.mq.workers_per_factory {
            // let mqc = mq.to_owned();
            let mut factory = IbmMqFactory::new(Arc::new(mq.to_owned()));
            let prod = producer.to_owned();
            let run = running.to_owned();
            let start = starting.to_owned();

            tokio::spawn(async move {
                debug!("Start get messages loop...");
                // let mq = mq.clone();
                let _ = factory.connect().unwrap();
                let _ = factory.open_queue().unwrap();
                read_messages(&factory, prod.to_owned(), run.to_owned(), start.to_owned()).await;
            });
        }
    });

    let mut send_factory = IbmMqFactory::new(Arc::new(config.mq.audit.clone()));
    let _ = send_factory.connect().unwrap();
    let _ = send_factory.open_queue().unwrap();

    let web_data = web::Data::new((
        producer.to_owned(),
        Arc::new(send_factory),
        running,
        starting
    ));
    HttpServer::new( move || {
        App::new()
            .app_data(web_data.clone())
            .wrap(middleware::Logger::default())
            .service(web::scope("/").configure(init_routes))
    })
    // .workers(24)
    .shutdown_timeout(shutdown_timeout)
    // .client_timeout(1000) // 1 second timeout
    .bind(bind_url)?
    .run()
    .await
}
