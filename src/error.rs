use crate::json::Json;
use crate::request::Request;
use crate::response::Response;
use crate::types::Body;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Error {
    code: u16,
    msg: String,
}

pub fn not_found(req: &Request) -> Box<dyn Body> {
    Box::new(Json(Error {
        code: 404,
        msg: "".to_string(),
    }))
}

pub fn is_invalid(req: &Request) -> Box<dyn Body> {
    Box::new(Json(Error {
        code: 400,
        msg: "".to_string(),
    }))
}
