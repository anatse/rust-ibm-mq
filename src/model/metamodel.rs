use serde::{Serialize, Deserialize};
use std::option::Option;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub enum EventCriticality {
    CRITICAL,
    UNCRITICAL,
    ERROR,
}

#[derive(Serialize, Deserialize)]
pub struct MetaModelParam {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<HashMap<String, String>>,
    #[serde(rename = "isMandatory")]
    pub mandatory: bool,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<u16>,
}

#[derive(Serialize, Deserialize)]
pub struct MetaModelEvent {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<HashMap<String, String>>,
    #[serde(rename = "criticality", skip_serializing_if = "Option::is_none")]
    pub criticality: Option<EventCriticality>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, MetaModelParam>>,
}

#[derive(Serialize, Deserialize)]
pub struct MetaModelGroup {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<HashMap<String, String>>,
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<HashMap<String, MetaModelEvent>>,
}

#[derive(Serialize, Deserialize)]
pub struct MetaModel {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<HashMap<String, String>>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<HashMap<String, MetaModelGroup>>,
    #[serde(rename = "subsystemCode", skip_serializing_if = "String::is_empty")]
    pub subsystem_code: String,
    #[serde(rename = "earVersion", skip_serializing_if = "String::is_empty")]
    pub ear_version: String,
    #[serde(rename = "earName", skip_serializing_if = "String::is_empty")]
    pub ear_name: String,
    #[serde(rename = "version", skip_serializing_if = "String::is_empty")]
    pub version: String,
}