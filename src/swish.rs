use crate::config::Config;
use crate::cors::Cors;
use crate::entities::is_request_url;
use crate::error::{is_invalid, is_unauthorized};
use crate::http::Method;
use crate::request::{parse, Request};
use crate::response::Response;
use crate::router::{handler_exec, Handler, Router};
use crate::validater::Validater;

use std::net::{TcpListener, TcpStream};

pub struct Swish {
    pub router: Router,
    pub config: Config,
    pub validater: Validater,
}

impl Swish {
    pub fn new() -> Swish {
        Swish {
            router: Router::new(),
            config: Config::new(),
            validater: Default::default(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.router.register(path, Method::GET, handler)
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.router.register(path, Method::POST, handler)
    }

    pub fn set_cors_as(&mut self, cors: Cors) {
        self.validater.set_cors(cors)
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
        let (is_valid, msg) = self.validater.validate_request(&req);
        if is_valid {
            let res = self.search(&mut req);
            res.send(stream);
        } else {
            let res = handler_exec(is_unauthorized, &req);
            res.send(stream);
        }
    }

    fn search(&mut self, req: &mut Request) -> Response {
        if req.is_valid() && is_request_url(&req.path) {
            let (handler, req) = self.router.get_handler_and_req(req);
            handler_exec(handler, &req)
        } else {
            handler_exec(is_invalid, &*req)
        }
    }
}
