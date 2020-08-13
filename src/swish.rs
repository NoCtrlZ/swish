use crate::entities::is_request_url;
use crate::error::{is_invalid, not_found};
use crate::global::Config;
use crate::matcher::match_with;
use crate::request::{parse, Request};
use crate::response::{write, Response};
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
        let res = self.search(&mut req);
        write(&res.compile(), stream)
    }

    pub fn search(&mut self, mut req: &mut Request) -> Response {
        if req.is_valid() && is_request_url(&req.path) {
            for route in &self.router.routes {
                if match_with(&mut req, route) {
                    return Response {
                        status: 200,
                        body: (route.handler)(&*req).compile(),
                    };
                } else {
                    continue;
                };
            }
            Response {
                status: 404,
                body: (not_found)(&*req).compile(),
            }
        } else {
            Response {
                status: 400,
                body: (is_invalid)(&*req).compile(),
            }
        }
    }
}
