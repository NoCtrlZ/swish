use crate::matcher::match_with;
use crate::router::{Router, Handler};
use crate::request::{Request, parse};
use crate::error::not_found;
use crate::response::{response, write};

use std::net::{TcpStream, TcpListener};
use std::io::prelude::*;


pub struct Swish {
    pub router: Router,
}

impl Swish {
    pub fn new() -> Swish {
        Swish {
            router: Router {
                routes: Vec::new(),
            }
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
        let req = parse(stream);
        // println!("{:?}", req);
        let handler = self.search(&req);
        let contents = response(handler, req);
        write(&contents, stream)
    }

    pub fn search(&mut self, req: &Request) -> Handler {
        for route in &self.router.routes {
            if match_with(&req, route) {
                return route.handler
            } else {
                continue;
            };
        }
        not_found
    }
}

