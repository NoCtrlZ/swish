use crate::matcher;
use crate::parser::parse;
use crate::router::{Router, Handler};
use crate::request::{Request};

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
        for route in &self.router.routes {
            if route.method == req.method && route.path == req.path {
                self.response(stream, route.handler, req);
                break;
            }
        }
    }

    fn response(&mut self, stream: &mut TcpStream, handler: Handler, req: Request) {
        let response = handler(&req.path);
        println!("response is here");
        stream.write(response.as_bytes()).expect("fail to write bytes");
        stream.flush().expect("fail to flush stream");
    }
}
