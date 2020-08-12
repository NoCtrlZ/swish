use crate::entities::is_request_url;
use crate::error::{is_invalid, not_found};
use crate::global::Config;
use crate::matcher::match_with;
use crate::request::{parse, Request};
use crate::response::{response, write};
use crate::router::{Handler, Router};

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub struct Swish {
    pub router: Router,
    pub config: Config,
}

impl Swish {
    pub fn new() -> Swish {
        Swish {
            router: Router { routes: Vec::new() },
            config: Config::new(),
        }
    }

    pub fn swish(&mut self, path: &str, method: &str, handler: Handler) {
        self.router.register(path, method, handler)
    }

    pub fn bish(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:3000").expect("fail to bind tcp listener");
        for stream in listener.incoming() {
            self.handle(&mut stream.expect("fail to read stream"));
        }
    }

    fn handle(&mut self, stream: &mut TcpStream) {
        let mut req = parse(stream);
        println!("{:?}", req);
        let handler = self.search(&mut req);
        let res = response(handler, req);
        write(&res.compile(), stream)
    }

    pub fn search(&mut self, mut req: &mut Request) -> Handler {
        if req.is_valid() && is_request_url(&req.path) {
            for route in &self.router.routes {
                if match_with(&mut req, route) {
                    return route.handler;
                } else {
                    continue;
                };
            }
            not_found
        } else {
            is_invalid
        }
    }
}
