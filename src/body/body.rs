use crate::error::{Error, ReturnError};
use crate::http::StatusCode;
use crate::body::json::Json;

use serde_json::json;
use serde::Serialize;

pub trait Body {
    fn status(&self) -> StatusCode;
    fn content_type(&self) -> String;
    fn body(&self) -> String;
}

impl<T: Serialize> Body for Json<T> {
    fn content_type(&self) -> String {
        "application/json".to_string()
    }

    fn status(&self) -> StatusCode {
        StatusCode::Ok
    }

    fn body(&self) -> String {
        json!(self.0).to_string()
    }
}

impl Body for Error {
    fn content_type(&self) -> String {
        "application/json".to_string()
    }

    fn status(&self) -> StatusCode {
        self.0.get_status_code()
    }

    fn body(&self) -> String {
        json!(ReturnError {
            status: false,
            code: self.0.status_code.get_code_number(),
            message: self.0.message.clone(),
        })
        .to_string()
    }
}
