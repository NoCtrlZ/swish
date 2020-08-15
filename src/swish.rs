use crate::entities::is_request_url;
use crate::error::{is_invalid, not_found};
use crate::global::Config;
use crate::http::Method;
use crate::matcher::match_with;
use crate::request::{parse, Request};
use crate::response::{write, Response};
use crate::router::{handler_exec, Handler, Router};

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

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.router.register(path, Method::GET, handler)
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.router.register(path, Method::POST, handler)
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
        let mut res = self.search(&mut req);
        let contents = self.compose(&mut res);
        write(&res.compile(), stream)
    }

    fn compose(&self, res: &mut Response) -> String {
        let header = format!("Content-Type: {}; {}", res.ctype, self.config.get_charset());
        res.set_header(&header);
        res.compile()
    }

    pub fn search(&mut self, mut req: &mut Request) -> Response {
        if req.is_valid() && is_request_url(&req.path) {
            for route in &self.router.routes {
                if match_with(&mut req, route) {
                    return handler_exec(route.handler, &*req, 200);
                } else {
                    continue;
                };
            }
            handler_exec(not_found, &*req, 404)
        } else {
            handler_exec(not_found, &*req, 400)
        }
    }
}
