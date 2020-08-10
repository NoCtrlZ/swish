use crate::response::Response;
use crate::request::Request;

#[derive(Debug)]
pub struct ReqErr {
    pub msg: String
}

impl ReqErr {
    pub fn new(msg: &str) -> ReqErr {
        ReqErr {
            msg: msg.to_string(),
        }
    }
}

pub fn not_found(req: Request) -> Response {
    Response {
        status: "404".to_string(),
        body: format!("{}{}", req.path, " was not found"),
    }
}

pub fn is_invalid(req: Request) -> Response {
    Response {
        status: "400".to_string(),
        body: "request is not valid".to_string(),
    }
}
