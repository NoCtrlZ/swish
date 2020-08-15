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
        write(&contents, stream)
    }

    fn compose(&self, res: &mut Response) -> String {
        let header = format!("Content-Type: {}; {}", res.ctype, self.config.get_charset());
        res.set_header(&header);
        res.compile()
    }

    fn search(&mut self, mut req: &mut Request) -> Response {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::Json;
    use crate::types::Body;
    use serde::{Deserialize, Serialize};

    #[test]
    fn response_test() {
        let mut tester = Tester::new(swish2());
        let res1 = tester.get("/path");
        let res2 = tester.get("/greet");
        let res3 = tester.get("/no_route");
        let res4 = tester.get("shouldn't be return *");
        let res5 = tester.get("//gsaj");
        let res6 = tester.get("");
        let res7 = tester.get("/user/23");
        assert_eq!(res1, "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{\"code\":200,\"data\":\"path request\"}");
        assert_eq!(res2, "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{\"code\":200,\"data\":\"hi good morning\"}");
        assert_eq!(res3, "HTTP/1.1 404 Not Found\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{\"code\":404,\"msg\":\"\"}");
        assert_eq!(res4, "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{\"code\":404,\"msg\":\"\"}");
        assert_eq!(res5, "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{\"code\":404,\"msg\":\"\"}");
        assert_eq!(res6, "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{\"code\":404,\"msg\":\"\"}");
        assert_eq!(res7, "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{\"code\":200,\"data\":\"user id is 23\"}");
    }

    pub struct Tester {
        server: Swish,
    }

    impl Tester {
        fn new(swish: Swish) -> Tester {
            Tester { server: swish }
        }

        pub fn get(&mut self, path: &str) -> String {
            let mut req = Request {
                method: Method::GET,
                path: path.to_string(),
                header: "".to_string(),
                body: "".to_string(),
                param: "".to_string(),
            };
            let mut res = self.server.search(&mut req);
            self.server.compose(&mut res)
        }
    }

    #[derive(Deserialize, Serialize)]
    struct Sample {
        code: u16,
        data: String,
    }

    fn path_handler(req: &Request) -> Box<dyn Body> {
        Box::new(Json(Sample {
            code: 200,
            data: "path request".to_string(),
        }))
    }

    fn greet_handler(req: &Request) -> Box<dyn Body> {
        Box::new(Json(Sample {
            code: 200,
            data: "hi good morning".to_string(),
        }))
    }

    fn user_id_handler(req: &Request) -> Box<dyn Body> {
        Box::new(Json(Sample {
            code: 200,
            data: format!("{}{}", "user id is ".to_string(), req.param),
        }))
    }

    fn user_register_handler(req: &Request) -> Box<dyn Body> {
        Box::new(Json(Sample {
            code: 200,
            data: format!("{}{}", "user id is ".to_string(), req.param),
        }))
    }

    fn swish2() -> Swish {
        let mut swish = Swish::new();
        swish.get("/path", path_handler);
        swish.get("/greet", greet_handler);
        swish.get("/user/:id", user_id_handler);
        swish.post("/user/register", user_register_handler);
        swish
    }
}
