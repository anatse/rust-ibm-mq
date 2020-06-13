#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use serde::Deserialize;
use hocon::HoconLoader;

#[derive(Deserialize, Clone, Debug)]
pub struct MqConfig {
    pub enabled: bool,
    pub server_name: Option<String>,
    pub server_port: Option<u16>,
    pub queue_manager: Option<String>,
    pub channel_name: Option<String>,
    pub target_queue: Option<String>,
    pub user_id: Option<String>,
    pub password: Option<String>,
    pub ssl_key_repos_stem: Option<String>,
    pub cipher_spec: Option<String>,
    pub certificate_label: Option<String>,
    pub oscp_url: Option<String>,
    pub fips_required: bool,
    pub suite_b: Vec<i32>,
    pub cert_val_policy: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
    pub keep_alive: usize,
    pub shutdown_timeout: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TrustStore {
    pub store_type: Option<String>,
    pub location: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KeyStore {
    pub store_type: Option<String>,
    pub location: Option<String>,
    pub password: Option<String>,
    pub key_password: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Ssl {
    pub enabled: bool,
    pub enabled_protocols: Option<String>,
    pub endpoint_identification_algorithm: Option<String>,
    pub truststore: TrustStore,
    pub keystore: KeyStore,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Topics {
    pub events_uncritical: String,
    pub events_critical: String,
    pub meta_model: String,
    pub events_batch: String,
}

/// Kafka connection configuration
#[derive(Deserialize, Debug, Clone)]
pub struct KafkaConfig {
    pub client_id: String,
    pub socket_timeout_ms: String,
    pub queue_buffering_max_messages: String,
    pub enable_idempotence: String,
    pub message_send_max_retries_ms: String,
    pub retry_backoff_ms: String,
    pub bootstrap_servers: String,
    pub security_protocol: String,
    pub message_timeout: String,
    pub topics: Topics,
    pub ssl: Ssl,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Mq {
    /// Number of workers per factory
    pub workers_per_factory: u16,
    /// List of MQ servers configurations
    pub factories: Vec<MqConfig>,
    /// Audit factory
    pub audit: MqConfig,
}

/// Struct describes application configuration
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    /// HTTP server configuration
    pub http: HttpConfig,
    /// Kafka configuration
    pub kafka: KafkaConfig,
    /// Mq configuration
    pub mq: Mq,
}

impl Config {
    pub fn load(configName: &str) -> Self {
        let doc: Config = HoconLoader::new()
            .load_file(configName).expect(format!("Unable to load config from file {}", configName).as_str())
            .hocon().and_then(move |conf| {
            conf.resolve()
        }).expect("Unable to resolve configuration as config");

        doc
    }
}