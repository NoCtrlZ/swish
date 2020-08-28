use crate::body::Body;
use crate::http::StatusCode;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Error(pub ValidateError);

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidateError {
    pub status_code: StatusCode,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReturnError {
    pub status: bool,
    pub status_code: u16,
    pub message: String,
}

impl Body for Error {
    fn content_type(&self) -> String {
        "application/json".to_string()
    }

    fn status(&self) -> StatusCode {
        self.0.status_code.clone()
    }

    fn contents(&self) -> String {
        json!(ReturnError {
            status: false,
            status_code: self.0.status_code.get_code_number(),
            message: self.0.message.to_string()
        }).to_string()
    }
}
