use crate::body::Body;
use crate::header::compose_header;
use crate::request::Request;
use crate::response::Response;

pub type Handler = fn(&Request) -> Box<dyn Body>;

pub fn handler_exec(handler: Handler, req: &Request) -> Response {
    let res_contents = (handler)(req);
    Response {
        header: compose_header(res_contents.status(), res_contents.content_type()),
        body: res_contents,
    }
}
