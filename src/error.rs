use crate::request::Request;
use crate::response::Response;

pub fn not_found(req: Request) -> Response {
    Response {
        status: 404,
        body: "".to_string(),
    }
}

pub fn is_invalid(req: Request) -> Response {
    Response {
        status: 400,
        body: "".to_string(),
    }
}
