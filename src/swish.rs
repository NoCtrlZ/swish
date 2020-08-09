use crate::matcher::match_with;
use crate::parser::parse;
use crate::router::{Router, Handler};
use crate::request::Request;
use crate::error::not_found;

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
        let contents = self.response(handler, req);
        self.write(&contents, stream)
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

    pub fn response(&mut self, handler: Handler, req: Request) -> String {
        handler(&req.path)
    }

    fn write(&mut self, contents: &str, stream: &mut TcpStream) {
        stream.write(contents.as_bytes()).expect("fail to write bytes");
        stream.flush().expect("fail to flush stream");
    }
}
