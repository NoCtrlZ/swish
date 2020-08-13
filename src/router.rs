use crate::entities::is_route_url;
use crate::request::Request;
use crate::response::Response;
use crate::types::Body;

pub type Handler = fn(&Request) -> Box<dyn Body>;

pub struct Route {
    pub path: String,
    pub method: String,
    pub handler: Handler,
}

pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    pub fn register(&mut self, path: &str, method: &str, handler: Handler) {
        if !is_route_url(&path) {
            panic!("invalid routing");
        }
        let route = Route {
            path: path.to_string(),
            method: method.to_string(),
            handler: handler,
        };
        self.routes.push(route)
    }
}

pub fn handler_exec(handler: Handler, req: &Request, statud_code: u16) -> Response {
    let body = (handler)(req);
    Response {
        status_code: statud_code,
        ctype: body.get_content_type(),
        header: "".to_string(),
        body: body.compile(),
    }
}
