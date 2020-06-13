#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use std::option::Option;
use std::fmt;
use serde::export::Formatter;
use chrono::NaiveDateTime;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct AuditEvent {
    // Required fields
    #[serde(rename = "eventDateTime")]
    pub event_date_time: u128,
    #[serde(rename = "uuid", skip_serializing_if = "String::is_empty")]
    pub uuid: String,
    #[serde(rename = "subsystemCode", skip_serializing_if = "String::is_empty")]
    pub subsystem_code: String,
    #[serde(rename = "metaModelVersion", skip_serializing_if = "String::is_empty")]
    pub meta_model_version: String,
    #[serde(rename = "isSuccess")]
    pub is_success: bool,
    #[serde(rename = "groupCode", skip_serializing_if = "String::is_empty")]
    pub group_code: String,
    #[serde(rename = "eventCode", skip_serializing_if = "String::is_empty")]
    pub event_code: String,
    // Optional fields]
    #[serde(rename = "sector", skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "userLogin", skip_serializing_if = "Option::is_none")]
    pub user_login: Option<String>,
    #[serde(rename = "workstationIp", skip_serializing_if = "Option::is_none")]
    pub workstation_ip: Option<String>,
    #[serde(rename = "isInitContext", skip_serializing_if = "Option::is_none")]
    pub is_init_context: Option<bool>,
    #[serde(rename = "categoryCode", skip_serializing_if = "Option::is_none")]
    pub category_code: Option<String>,
    #[serde(rename = "earName", skip_serializing_if = "Option::is_none")]
    pub ear_name: Option<String>,
    #[serde(rename = "earVersion", skip_serializing_if = "Option::is_none")]
    pub ear_version: Option<String>,
    #[serde(rename = "employeeLogin", skip_serializing_if = "Option::is_none")]
    pub employee_login: Option<String>,
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "terBankCode", skip_serializing_if = "Option::is_none")]
    pub ter_bank_code: Option<String>,
    #[serde(rename = "gosbCode", skip_serializing_if = "Option::is_none")]
    pub gosb_code: Option<String>,
    #[serde(rename = "vspCode", skip_serializing_if = "Option::is_none")]
    pub vsp_code: Option<String>,
    #[serde(rename = "osbCode", skip_serializing_if = "Option::is_none")]
    pub osb_code: Option<String>,
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "auditContextUUID", skip_serializing_if = "Option::is_none")]
    pub audit_context_uuid: Option<String>,
    #[serde(rename = "criticality", skip_serializing_if = "Option::is_none")]
    pub criticality: Option<String>,
    #[serde(rename = "oper_id", skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, String>>,
}

pub struct AuditEventBuilder {
    event: AuditEvent,
}

impl AuditEventBuilder {
    #[allow(dead_code)]
    pub fn new() -> AuditEventBuilder {
        AuditEventBuilder {
            event: AuditEvent {
                event_date_time: SystemTime::now().duration_since(UNIX_EPOCH).expect("Error getting current time").as_millis(),
                uuid: "".to_string(),
                subsystem_code: "".to_string(),
                meta_model_version: "".to_string(),
                is_success: false,
                group_code: "".to_string(),
                event_code: "".to_string(),
                sector: None,
                session_id: None,
                user_login: None,
                workstation_ip: None,
                is_init_context: None,
                category_code: None,
                ear_name: None,
                ear_version: None,
                employee_login: None,
                channel: None,
                ter_bank_code: None,
                gosb_code: None,
                vsp_code: None,
                osb_code: None,
                request_id: None,
                audit_context_uuid: None,
                criticality: None,
                operation_id: None,
                params: None
            }
        }
    }

    pub fn event_date_time(&mut self, time: u128) -> &mut AuditEventBuilder {
        self.event.event_date_time = time;
        self
    }

    pub fn uuid(&mut self, uuid: String) -> &mut AuditEventBuilder {
        self.event.uuid = uuid;
        self
    }

    pub fn subsystem_code(&mut self, code: String) -> &mut AuditEventBuilder {
        self.event.subsystem_code = code;
        self
    }

    pub fn meta_model_version(&mut self, code: String) -> &mut AuditEventBuilder {
        self.event.meta_model_version = code;
        self
    }

    pub fn success(&mut self, flag: bool) -> &mut AuditEventBuilder {
        self.event.is_success = flag;
        self
    }

    pub fn group_code(&mut self, code: String) -> &mut AuditEventBuilder {
        self.event.group_code = code;
        self
    }

    pub fn event_code(&mut self, code: String) -> &mut AuditEventBuilder {
        self.event.event_code = code;
        self
    }

    pub fn sector(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.sector = code;
        self
    }

    pub fn session_id(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.session_id = code;
        self
    }

    pub fn user_login(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.user_login = code;
        self
    }

    pub fn workstation_ip(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.workstation_ip = code;
        self
    }

    pub fn init_context(&mut self, flag: Option<bool>) -> &mut AuditEventBuilder {
        self.event.is_init_context = flag;
        self
    }

    pub fn category_code(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.category_code = code;
        self
    }

    pub fn ear_name(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.ear_name = code;
        self
    }

    pub fn ear_version(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.ear_version = code;
        self
    }

    pub fn employee_login(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.employee_login = code;
        self
    }

    pub fn channel(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.channel = code;
        self
    }

    pub fn ter_bank_code(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.ter_bank_code = code;
        self
    }

    pub fn gosb_code(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.gosb_code = code;
        self
    }

    pub fn vsp_code(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.vsp_code = code;
        self
    }

    pub fn osb_code(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.osb_code = code;
        self
    }

    pub fn request_id(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.request_id = code;
        self
    }

    pub fn audit_context_uuid(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.audit_context_uuid = code;
        self
    }

    pub fn criticality(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.criticality = code;
        self
    }

    pub fn oper_id(&mut self, code: Option<String>) -> &mut AuditEventBuilder {
        self.event.operation_id = code;
        self
    }

    pub fn params(&mut self, params: Option<HashMap<String, String>>) -> &mut AuditEventBuilder {
        self.event.params = params;
        self
    }

    pub fn build(&self) -> AuditEvent {
        AuditEvent {
            event_date_time: self.event.event_date_time,
            uuid: self.event.uuid.to_owned(),
            subsystem_code: self.event.subsystem_code.to_owned(),
            meta_model_version: self.event.meta_model_version.to_owned(),
            is_success: self.event.is_success,
            group_code: self.event.group_code.to_owned(),
            event_code: self.event.event_code.to_owned(),
            sector: self.event.sector.to_owned(),
            session_id: self.event.session_id.to_owned(),
            user_login: self.event.user_login.to_owned(),
            workstation_ip: self.event.workstation_ip.to_owned(),
            is_init_context: self.event.is_init_context,
            category_code: self.event.category_code.to_owned(),
            ear_name: self.event.ear_name.to_owned(),
            ear_version: self.event.ear_version.to_owned(),
            employee_login: self.event.employee_login.to_owned(),
            channel: self.event.channel.to_owned(),
            ter_bank_code: self.event.ter_bank_code.to_owned(),
            gosb_code: self.event.gosb_code.to_owned(),
            vsp_code: self.event.vsp_code.to_owned(),
            osb_code: self.event.osb_code.to_owned(),
            request_id: self.event.request_id.to_owned(),
            audit_context_uuid: self.event.audit_context_uuid.to_owned(),
            criticality: self.event.criticality.to_owned(),
            operation_id: self.event.operation_id.to_owned(),
            params: self.event.params.to_owned(),
        }
    }
}

impl fmt::Display for AuditEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "AuditEvent:\n    \
                    uuid: {}\n    \
                    time: {}\n    \
                    subsystem_code: {}\n    \
                    group_code: {}\n    \
                    event_code: {}",
            self.uuid,
            NaiveDateTime::from_timestamp((self.event_date_time / 1000) as i64, (self.event_date_time % 1000) as u32),
            self.subsystem_code,
            self.group_code,
            self.event_code
        )
    }
}
