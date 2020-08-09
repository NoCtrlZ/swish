use crate::swish::Swish;
use crate::response::response;
use crate::request::Request;

pub struct Client {
    pub server: Swish
}

impl Client {
    pub fn new(swish: Swish) -> Client {
        Client {
            server: swish,
        }
    }

    pub fn get(&mut self, path: &str) -> String {
        let req = Request {
            method: "GET".to_string(),
            path: path.to_string(),
        };
        let handler = self.server.search(&req);
        response(handler, req)
    }
}
