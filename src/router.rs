use crate::entities::is_route_url;

pub type Handler = fn(&str) -> String;

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
