#![allow(dead_code)]

use crate::model::event::AuditEvent;
use std::vec::Vec;
use futures::FutureExt;
use futures::channel::oneshot;
use rdkafka::error::KafkaError;
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use rdkafka::producer::{FutureProducer, FutureRecord, DeliveryFuture};
use crate::config::config::{Config, KafkaConfig};
use rdkafka::ClientConfig;
use serde_json;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use crate::model::metamodel::MetaModel;

use log::{debug, info};
use std::sync::Arc;
use std::thread;

/// Класс для работы с кафкой
pub struct KafkaProducer {
    producer: Box<FutureProducer>,
    config: Arc<Config>,
}

/// Функция превращает длительность в миллисекунды по аналогии с Java System.currentTimeMillis()
pub fn duration_to_millis(duration: Duration) -> u64 {
    let nanos = duration.subsec_nanos() as u64;
    duration.as_secs() * 1000 + nanos/1_000_000
}

/// Функция превращает системное время в миллисекунда по аналогии с Java System.currentTimeMillis()
/// ```
///   let timestamp = millis_to_epoch(SystemTime::now());
/// ```
pub fn millis_to_epoch(time: SystemTime) -> i64 {
    let duration_since_epoch = time.duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0));

    duration_to_millis(duration_since_epoch) as i64
}

impl KafkaProducer {
    pub fn new(config: Arc<Config>) -> Self {
        debug!("Trying to create kafka connection...");

        let mut cfg = ClientConfig::new();
        cfg
            .set("client.id", config.kafka.client_id.as_str())
            .set("socket.timeout.ms", config.kafka.socket_timeout_ms.as_str())
            .set("queue.buffering.max.messages", config.kafka.queue_buffering_max_messages.as_str())
            .set("enable.idempotence", config.kafka.enable_idempotence.as_str())
            .set("message.send.max.retries", config.kafka.message_send_max_retries_ms.as_str())
            .set("retry.backoff.ms", config.kafka.retry_backoff_ms.as_str())
            .set("bootstrap.servers", config.kafka.bootstrap_servers.as_str())
            .set("message.timeout.ms", config.kafka.message_timeout.as_str())
            // .set("auto.create.topics.enable", "false")
            .set("enable.auto.commit", "false");

        if config.kafka.ssl.enabled {
            let ssl = config.kafka.ssl.to_owned();
            if let Some(endpoint_identification_algorithm) = ssl.endpoint_identification_algorithm {
                cfg.set("ssl.endpoint.identification.algorithm",
                        endpoint_identification_algorithm.as_str());
            }

            if let Some(enabled_protocols) = ssl.enabled_protocols {
                cfg.set("ssl.enabled.protocols", enabled_protocols.as_str());
            }

            if let Some(store_type) = ssl.keystore.store_type.to_owned() {
                cfg.set("ssl.keystore.type", store_type.as_str());
            }

            if let Some(location) = ssl.keystore.store_type.to_owned() {
                cfg.set("ssl.keystore.location", location.as_str());
            }

            if let Some(password) = ssl.keystore.password.to_owned() {
                cfg.set("ssl.keystore.password", password.as_str());
            }

            if let Some(key_password) = ssl.keystore.key_password.to_owned() {
                cfg.set("ssl.keystore.key_password", key_password.as_str());
            }

            if let Some(store_type) = ssl.truststore.password.to_owned() {
                cfg.set("ssl.truststore.type", store_type.as_str());
            }

            if let Some(location) = ssl.truststore.location.to_owned() {
                cfg.set("ssl.truststore.location", location.as_str());
            }

            if let Some(password) = ssl.truststore.password.to_owned() {
                cfg.set("ssl.truststore.password", password.as_str());
            }
        }

        let producer: FutureProducer = cfg.create().expect("Producer creation error");
        KafkaProducer {
            producer: Box::new(producer),
            config,
        }
    }

    /// Функция отправляет строковый контент в кафку в топик
    /// # Аргументы
    /// * `_payload` - Строка, которая будет отправлена в кафку
    /// * `_topic` - Топик в который необходимо отправить
    pub fn send<'a, 'b>(&'b self, _payload: &'a str, _topic: &'a str) -> DeliveryFuture {
        let rec = FutureRecord::<(), str>::to(_topic)
            .payload(_payload)
            .timestamp(millis_to_epoch(SystemTime::now()));

            self.producer.send_result(rec).unwrap()
    }

    /// Отправка события аудита в кафку
    /// # Аргументы
    /// * `event` - событие аудита
    /// Возвращает Future с результатом отправки
    pub fn send_event(&self, event: &AuditEvent, critical: bool) -> DeliveryFuture {
        self.send_event_raw(
        serde_json::to_string(&event)
                    .expect(
                        format!("Error serializing event {}", event.to_string()).as_str()
                    )
                    .as_str(),
            critical
        )
    }

    /// Send event as string w/o serialization/deserialization
    pub fn send_event_raw(&self, event: &str, critical: bool) -> DeliveryFuture {
        info!("[{:?}] send_event_raw start", thread::current().id());
        self.send(event,  if critical {
            info!("send_event_raw end");
            self.config.kafka.topics.events_critical.as_str()
        } else {
            info!("send_event_raw end");
            self.config.kafka.topics.events_uncritical.as_str()
        })
    }

    ///  Отправляет пачку событий аудита
    /// # Аргументы
    /// * `events` - события аудита
    /// Возвращает Future с результатом отправки
    pub fn send_batch_event(&self, events: &Vec<AuditEvent>) -> DeliveryFuture {
        self.send_batch_event_raw(
            serde_json::to_string(&events)
                .expect(
                    format!("Error serializing events batch, size: {}", events.len()).as_str()
                )
                .as_str()
        )
    }

    pub fn send_batch_event_raw(&self, events: &str) -> DeliveryFuture {
        self.send(events, self.config.kafka.topics.events_batch.as_str())
    }

    pub fn send_meta_model(&self, meta_model: &MetaModel) -> DeliveryFuture {
        self.send_meta_model_raw(
            serde_json::to_string(&meta_model)
                .expect(
                    format!("Error serializing meta_model: {}", meta_model.version).as_str()
                )
                .as_str()
        )
    }

    pub fn send_meta_model_raw(&self, meta_model: &str) -> DeliveryFuture {
        self.send(meta_model, self.config.kafka.topics.meta_model.as_str())
    }
}

