use crate::body::ValidateError;
use crate::body::{Body, Error};
use crate::http::StatusCode;
use crate::request::Request;

use serde::{Deserialize, Serialize};

pub fn is_not_found(_: &Request) -> Box<dyn Body> {
    Box::new(Error(ValidateError {
        status_code: StatusCode::NotFound,
        message: "page is not found".to_string(),
    }))
}
