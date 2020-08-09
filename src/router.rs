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
        let route = Route {
            path: path.to_string(),
            method: method.to_string(),
            handler: handler,
        };
        self.routes.push(route)
    }
}
