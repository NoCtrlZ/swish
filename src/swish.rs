use crate::router;
use crate::matcher;
use std::net::{TcpStream, TcpListener};
use std::io::prelude::*;

use crate::router::{Router, Handler};

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

    pub fn swish(&mut self, path: &str, handler: Handler) {
        self.router.register(path, handler)
    }

    pub fn bish(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:3000").expect("fail to bind tcp listener");
        for stream in listener.incoming() {
            self.handle(&mut stream.expect("fail to read stream"));
        }
    }

    fn handle(&mut self, stream: &mut TcpStream) {
        println!("{:?}", stream);
    }
}
