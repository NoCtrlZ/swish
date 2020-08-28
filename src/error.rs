use crate::body::{Body, Error};
use crate::http::StatusCode;
use crate::request::Request;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorContents {
    pub status_code: StatusCode,
    pub message: String,
}

impl ErrorContents {
    pub fn get_status_code(&self) -> StatusCode {
        self.status_code.clone()
    }
}

pub enum ReqError {
    Empty,
    PathIsInvalid,
    IsNotAllowedOrigin,
    IsNotAllowedHeader,
    IsNotAllowedMethod,
    IsNotAllowedCredential,
    IsShaddy,
}

pub fn is_invalid(_: &Request) -> Box<dyn Body> {
    Box::new(Error(ErrorContents {
        status_code: StatusCode::BadRequest,
        message: "request is not valid".to_string(),
    }))
}

pub fn is_unauthorized(_: &Request) -> Box<dyn Body> {
    Box::new(Error(ErrorContents {
        status_code: StatusCode::Unauthorized,
        message: "header is not accepted".to_string(),
    }))
}

pub fn is_not_found(_: &Request) -> Box<dyn Body> {
    Box::new(Error(ErrorContents {
        status_code: StatusCode::NotFound,
        message: "page is not found".to_string(),
    }))
}
