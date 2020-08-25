use std::collections::HashMap;

use crate::body::Body;
use crate::entities::is_route_url;
use crate::error::is_not_found;
use crate::http::Method;
use crate::request::Request;
use crate::router::matcher::match_with;

pub type Handler = fn(&Request) -> Box<dyn Body>;

#[derive(Clone)]
pub struct Route {
    pub path: String,
    pub method: Method,
    pub handler: Handler,
}

pub struct Router {
    pub routes: HashMap<Method, Vec<Route>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn register(&mut self, path: &str, method: Method, handler: Handler) {
        if !is_route_url(&path) {
            panic!("invalid path request");
        }
        let route = Route {
            path: path.to_string(),
            method: method.clone(),
            handler: handler,
        };
        let mut entries = self
            .routes
            .entry(method.clone())
            .or_insert_with(|| vec![])
            .clone();
        entries.push(route);
        self.routes.insert(method.clone(), entries.to_vec());
    }

    pub fn get_handler_and_req(&self, req: &mut Request) -> (Handler, Request) {
        for route in &self.get_routes(req.method.clone()) {
            if match_with(req, route) {
                return (route.handler, req.clone());
            } else {
                continue;
            };
        }
        (is_not_found, req.clone())
    }

    pub fn get_routes(&self, method: Method) -> Vec<Route> {
        match self.routes.get(&method) {
            Some(routes) => routes.to_vec(),
            None => [].to_vec(),
        }
    }
}
