use crate::http::StatusCode;
use crate::error::ErrorContents;
use crate::body::Body;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Error(pub ErrorContents);

#[derive(Debug, Deserialize, Serialize)]
pub struct ReturnError {
    pub status: bool,
    pub code: u16,
    pub message: String,
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
