use crate::http::StatusCode;
use crate::json::Json;
use crate::request::Request;
use crate::response::Response;
use crate::types::Body;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Error(pub ErrorContents);

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorContents {
    pub status_code: StatusCode,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReturnError {
    pub status: bool,
    pub code: u16,
    pub message: String,
}

impl ErrorContents {
    pub fn get_status_code(&self) -> StatusCode {
        self.status_code.clone()
    }
}

pub fn is_invalid(req: &Request) -> Box<dyn Body> {
    Box::new(Error(ErrorContents {
        status_code: StatusCode::BadRequest,
        message: "request is not valid".to_string(),
    }))
}

pub fn is_not_found(req: &Request) -> Box<dyn Body> {
    Box::new(Error(ErrorContents {
        status_code: StatusCode::NotFound,
        message: "page is not found".to_string(),
    }))
}
