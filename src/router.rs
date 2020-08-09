pub type Handler = fn(url: &str) -> String;

pub struct Route {
    pub path: String,
    pub handler: Handler,
}

pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    pub fn register(&mut self, path: &str, handler: Handler) {
        let route = Route {
            path: path.to_string(),
            handler: handler,
        };
        self.routes.push(route)
    }
}
