#![allow(dead_code)]

use serde::Serialize;
use std::option::Option;

#[derive(Serialize)]
pub struct ErrorMessage {
    #[serde(alias = "uuid", skip_serializing_if = "Option::is_none")]
    uuid: Option<String>,
    #[serde(alias = "code", skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(alias = "system", skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(alias = "title", skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(alias = "text", skip_serializing_if = "String::is_empty")]
    text: String,
}

#[derive(Serialize)]
pub struct BaseResponse<T: Serialize> {
    #[serde(alias = "success")]
    success: bool,
    #[serde(alias = "body", skip_serializing_if = "Option::is_none")]
    body: Option<T>,
    #[serde(alias = "error", skip_serializing_if = "Option::is_none")]
    error: Option<ErrorMessage>,
}

impl <T> BaseResponse<T> where T: Serialize {
    pub fn error(error: ErrorMessage) -> Self {
       BaseResponse {
            success: false,
            body: None,
            error: Some(error),
        }
    }

    pub fn success(body: T) -> Self {
        BaseResponse {
            success: true,
            body: Some(body),
            error: None,
        }
    }
}

impl ErrorMessage {
    pub fn new(text: String) -> Self {
        ErrorMessage {
            uuid: None,
            code: None,
            system: None,
            title: None,
            text,
        }
    }

    pub fn build(&self) -> Self {
        ErrorMessage {
            uuid: self.uuid.to_owned(),
            code: self.code.to_owned(),
            system: self.system.to_owned(),
            title: self.title.to_owned(),
            text: self.text.to_owned(),
        }
    }

    pub fn with_uuid(&mut self, uuid: String) -> &mut ErrorMessage {
        self.uuid = Some(uuid);
        self
    }

    pub fn with_code(&mut self, code: String) -> &mut ErrorMessage {
        self.code = Some(code);
        self
    }

    pub fn with_system(&mut self, system: String) -> &mut ErrorMessage {
        self.system = Some(system);
        self
    }

    pub fn with_title(&mut self, title: String) -> &mut ErrorMessage {
        self.title = Some(title);
        self
    }
}