use serde::Deserialize;
use std::collections::HashMap;
use hocon::{HoconLoader, Hocon};
use std::any::Any;
use std::ops::Index;

#[derive(Deserialize, Debug)]
pub struct Topic {
    pub name: String,
    pub partitions: u16,
}

#[derive(Deserialize, Debug)]
pub struct KafkaSource {
    pub servers: Vec<String>,
    pub topic: Topic,
    pub consumer_group: String,
}

#[derive(Deserialize, Debug)]
pub struct DataSource {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub pool_max_size: u16,
}

#[derive(Deserialize, Debug)]
pub struct Destination {
    pub plugin: String,
}

#[derive(Deserialize, Debug)]
pub struct Transformer {
    pub source: KafkaSource,
    pub destination: Destination,
}

#[derive(Deserialize, Debug)]
pub struct App {
    pub transformers: Vec<Transformer>
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub app: App
}

#[derive(Debug)]
pub struct ConfigLoader {
    hocon: Hocon,
    app: App,
}

impl ConfigLoader {
    pub fn load_default() -> Self {
        let hocon = HoconLoader::new()
            .load_file("config/application.conf").expect("Error open file")
            .hocon().expect("Error loading config");

        let doc: AppConfig = hocon.clone()
            .resolve().expect("Error resolve config");

        ConfigLoader {
            hocon: hocon,
            app: doc.app,
        }
    }

    pub fn load_from_file(file: String) -> Self {
        let hocon = HoconLoader::new()
            .load_file(file).expect("Error open file")
            .hocon().expect("Error loading config");

        let doc: AppConfig = hocon.clone()
            .resolve().expect("Error resolve config");

        ConfigLoader {
            hocon: hocon,
            app: doc.app,
        }
    }

    pub fn get_destination_config(&self, transformer_index: u16) -> Hocon {
         self.hocon["app"]["transformers"]
            .index(transformer_index as usize)["destination"]["config"]
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{ConfigLoader, App};

    #[test]
    fn configuration_test() {
        let doc = ConfigLoader::load_default();
        println!("Loaded default config: {:?}", doc);

        assert_eq!(doc.app.transformers.len(), 1);
        assert_eq!(doc.app.transformers[0].source.servers[0], "localhost:091");
    }

    #[test]
    fn configuration_file_test() {
        let doc = ConfigLoader::load_from_file(String::from("config/application.conf"));
        println!("Loaded config from file: {:?}", doc);

        assert_eq!(doc.app.transformers.len(), 1);
        assert_eq!(doc.app.transformers[0].source.servers[0], "localhost:091");
    }

    #[test]
    fn configuration_hocon_test() {
        let doc = ConfigLoader::load_default();
        let cfg = doc.get_destination_config(0);
        print!("doc: {:?}", cfg);
    }
}
