use crate::config::Config;
use crate::cors::Cors;
use crate::http::Method;
use crate::request::parse;
use crate::router::{Handler, Router};
use crate::validater::{get_error_response, Validater};

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
        // println!("{:?}", req);
        let (is_valid, req_error) = self.validater.validate_request(&req);
        let res = match is_valid {
            true => self.router.get_response(&mut req),
            false => get_error_response(req_error),
        };
        res.send(stream);
    }
}
