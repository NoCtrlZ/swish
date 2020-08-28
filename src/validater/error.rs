use crate::response::Response;
use crate::http::StatusCode;
use crate::body::{Body, Error, ValidateError};
use crate::header::compose_header;

// #[derive(Debug, PartialEq, Eq)]
pub enum ReqError {
    Empty,
    IsInvalid,
    IsNotFound,
    IsNotAllowedOrigin,
    IsNotAllowedHeader,
    IsNotAllowedMethod,
    IsNotAllowedCredential,
}

pub fn get_error_response(etype: ReqError) -> Response {
    let error_body = get_error_body(etype);
    Response {
        header: compose_header(error_body.status(), error_body.content_type()),
        body: error_body,
    }
}

pub fn get_error_body(etype: ReqError) -> Box<dyn Body> {
    let status_code = match etype {
        ReqError::IsNotFound => StatusCode::NotFound,
        ReqError::IsInvalid => StatusCode::BadRequest,
        ReqError::IsNotAllowedOrigin | ReqError::IsNotAllowedHeader | ReqError::IsNotAllowedMethod | ReqError::IsNotAllowedCredential => StatusCode::Unauthorized,
        _ => StatusCode::Ok,
    };
    let message = match etype {
        ReqError::IsInvalid => "request is not valid",
        ReqError::IsNotFound => "page is not found",
        ReqError::IsNotAllowedOrigin => "origin is not allowed",
        ReqError::IsNotAllowedHeader => "header is not allowed",
        ReqError::IsNotAllowedMethod => "method is not allowed",
        ReqError::IsNotAllowedCredential => "credential is not allowed",
        _ => "",
    };
    Box::new(Error(ValidateError {
        status_code: status_code,
        message: message.to_string(),
    }))
}
