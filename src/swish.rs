use crate::config::HeaderConfig;
use crate::cors::Cors;
use crate::entities::is_request_url;
use crate::error::{is_invalid, is_not_found, is_unauthorized};
use crate::http::Method;
use crate::matcher::match_with;
use crate::request::{parse, Request};
use crate::response::{write, Response};
use crate::router::{Handler, Router};

use std::net::{TcpListener, TcpStream};

pub struct Swish {
    pub router: Router,
    pub config: HeaderConfig,
    pub cors: Option<Cors>,
}

impl Swish {
    pub fn new() -> Swish {
        Swish {
            router: Router { routes: Vec::new() },
            config: HeaderConfig::new(),
            cors: Default::default(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.router.register(path, Method::GET, handler)
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.router.register(path, Method::POST, handler)
    }

    pub fn set(&mut self, cors: Cors) {
        self.cors = Some(cors)
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
        if self.cors.is_some() {
            let res = self.search(&mut req);
        }
        let mut res = match &self.cors {
            Some(e) => {
                // todo should be clear the error reason by using msg
                let (is_valid, msg) = e.validate_request(&req);
                if is_valid {
                    self.search(&mut req)
                } else {
                    self.handler_exec(is_unauthorized, &req)
                }
            }
            None => self.search(&mut req),
        };
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
                    return self.handler_exec(route.handler, &*req);
                } else {
                    continue;
                };
            }
            self.handler_exec(is_not_found, &*req)
        } else {
            self.handler_exec(is_invalid, &*req)
        }
    }

    fn handler_exec(&self, handler: Handler, req: &Request) -> Response {
        let res_contents = (handler)(req);
        Response {
            status_code: res_contents.status(),
            ctype: res_contents.content_type(),
            header: "".to_string(),
            body: res_contents.body(),
            header_conf: self.config.clone(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::json::Json;
//     use crate::types::Body;
//     use serde::{Deserialize, Serialize};
//     use serde_json::json;

//     #[test]
//     fn get_response_test() {
//         let mut tester = Tester::new(swish2());
//         let res1 = tester.get("/path");
//         let res2 = tester.get("/greet");
//         let res3 = tester.get("/no_route");
//         let res4 = tester.get("shouldn't be return *");
//         let res5 = tester.get("//gsaj");
//         let res6 = tester.get("");
//         let res7 = tester.get("/user/23");
//         assert_eq!(res1, "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\nContent-Length: 34\r\n\r\n{\"code\":200,\"data\":\"path request\"}");
//         assert_eq!(res2, "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\nContent-Length: 37\r\n\r\n{\"code\":200,\"data\":\"hi good morning\"}");
//         assert_eq!(res3, "HTTP/1.1 404 Not Found\r\nContent-Type: application/json; charset=UTF-8\r\nContent-Length: 57\r\n\r\n{\"code\":404,\"message\":\"page is not found\",\"status\":false}");
//         assert_eq!(res4, "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json; charset=UTF-8\r\nContent-Length: 60\r\n\r\n{\"code\":400,\"message\":\"request is not valid\",\"status\":false}");
//         assert_eq!(res5, "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json; charset=UTF-8\r\nContent-Length: 60\r\n\r\n{\"code\":400,\"message\":\"request is not valid\",\"status\":false}");
//         assert_eq!(res6, "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json; charset=UTF-8\r\nContent-Length: 60\r\n\r\n{\"code\":400,\"message\":\"request is not valid\",\"status\":false}");
//         assert_eq!(res7, "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\nContent-Length: 35\r\n\r\n{\"code\":200,\"data\":\"user id is 23\"}");
//     }

//     #[test]
//     fn post_response_test() {
//         let mut tester = Tester::new(swish2());
//         let sample = Sample {
//             code: 1,
//             data: "shinsaku".to_string(),
//         };
//         let res1 = tester.post("/user/register", Box::new(Json(sample)));
//         assert_eq!(res1, "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\nContent-Length: 58\r\n\r\n{\"code\":200,\"data\":\"success register id: 1 msg: shinsaku\"}");
//     }

//     pub struct Tester {
//         server: Swish,
//     }

//     impl Tester {
//         fn new(swish: Swish) -> Tester {
//             Tester { server: swish }
//         }

//         pub fn get(&mut self, path: &str) -> String {
//             let mut req = Request {
//                 method: Method::GET,
//                 path: path.to_string(),
//                 header: "".to_string(),
//                 body: "".to_string(),
//                 param: "".to_string(),
//             };
//             let mut res = self.server.search(&mut req);
//             self.server.compose(&mut res)
//         }

//         pub fn post(&mut self, path: &str, body: Box<dyn Body>) -> String {
//             let mut req = Request {
//                 method: Method::POST,
//                 path: path.to_string(),
//                 header: "".to_string(),
//                 body: body.body(),
//                 param: "".to_string(),
//             };
//             let mut res = self.server.search(&mut req);
//             self.server.compose(&mut res)
//         }
//     }

//     #[derive(Deserialize, Serialize)]
//     struct Sample {
//         code: u16,
//         data: String,
//     }

//     fn path_handler(req: &Request) -> Box<dyn Body> {
//         Box::new(Json(Sample {
//             code: 200,
//             data: "path request".to_string(),
//         }))
//     }

//     fn greet_handler(req: &Request) -> Box<dyn Body> {
//         Box::new(Json(Sample {
//             code: 200,
//             data: "hi good morning".to_string(),
//         }))
//     }

//     fn user_id_handler(req: &Request) -> Box<dyn Body> {
//         Box::new(Json(Sample {
//             code: 200,
//             data: format!("{}{}", "user id is ".to_string(), req.param),
//         }))
//     }

//     fn user_register_handler(req: &Request) -> Box<dyn Body> {
//         let sample: Sample =
//             serde_json::from_str(&req.body).expect("fail to convert transaction to json");
//         Box::new(Json(Sample {
//             code: 200,
//             data: format!(
//                 "success register id: {} msg: {}",
//                 sample.code.to_string(),
//                 sample.data
//             ),
//         }))
//     }

//     fn swish2() -> Swish {
//         let mut swish = Swish::new();
//         swish.get("/path", path_handler);
//         swish.get("/greet", greet_handler);
//         swish.get("/user/:id", user_id_handler);
//         swish.post("/user/register", user_register_handler);
//         swish
//     }
// }
