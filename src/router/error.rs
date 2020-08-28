use crate::body::{Body, Error, ValidateError};
use crate::http::StatusCode;
use crate::request::Request;

pub fn is_not_found(_: &Request) -> Box<dyn Body> {
    Box::new(Error(ValidateError {
        status_code: StatusCode::NotFound,
        message: "page is not found".to_string(),
    }))
}
