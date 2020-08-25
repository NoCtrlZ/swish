use crate::config::Config;
use crate::cors::Cors;
use crate::entities::is_request_url;
use crate::error::{is_invalid, is_unauthorized};
use crate::header::compose_header;
use crate::http::Method;
use crate::request::{parse, Request};
use crate::response::Response;
use crate::router::{Handler, Router};

use std::net::{TcpListener, TcpStream};

pub struct Swish {
    pub router: Router,
    pub config: Config,
    pub cors: Option<Cors>,
}

impl Swish {
    pub fn new() -> Swish {
        Swish {
            router: Router::new(),
            config: Config::new(),
            cors: Default::default(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.router.register(path, Method::GET, handler)
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.router.register(path, Method::POST, handler)
    }

    pub fn set_cors_as(&mut self, cors: Cors) {
        self.cors = Some(cors)
    }

    pub fn set_address(&mut self, address: &str) {
        self.config.set_address(address)
    }

    pub fn bish(&mut self) {
        let listener =
            TcpListener::bind(self.config.get_origin()).expect("fail to bind tcp listener");
        for stream in listener.incoming() {
            self.handle(&mut stream.expect("fail to read stream"));
        }
    }

    fn handle(&mut self, stream: &mut TcpStream) {
        let mut req = parse(stream);
        println!("{:?}", req);
        let res = match &self.cors {
            Some(e) => {
                // todo should be clear the error reason by using msg, return handler
                let (is_valid, msg) = e.validate_request(&req);
                if is_valid {
                    self.search(&mut req)
                } else {
                    self.handler_exec(is_unauthorized, &req)
                }
            }
            None => self.search(&mut req),
        };
        res.send(stream);
    }

    fn search(&mut self, req: &mut Request) -> Response {
        if req.is_valid() && is_request_url(&req.path) {
            let (handler, req) = self.router.get_handler_and_req(req);
            self.handler_exec(handler, &req)
        } else {
            self.handler_exec(is_invalid, &*req)
        }
    }

    fn handler_exec(&self, handler: Handler, req: &Request) -> Response {
        let res_contents = (handler)(req);
        Response {
            header: compose_header(res_contents.status(), res_contents.content_type()),
            body: res_contents,
        }
    }
}
