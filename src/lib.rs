mod client;
mod entities;
mod error;
mod global;
mod http;
mod json;
mod matcher;
mod request;
mod response;
mod router;
mod swish;
mod types;

use crate::client::{request, Client};
use crate::json::Json;
use crate::request::Request;
use crate::response::Response;
use crate::swish::Swish;
use crate::types::Body;

extern crate regex;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn register_invalid_routing_test() {
        let mut swish = Swish::new();
        swish.swish("/=?", "GET", path_handler);
    }

    #[test]
    fn simple_path_test() {
        let mut client = Client::new(swish2());
        let res1 = client.get("/path");
        let res2 = client.get("/greet");
        let res3 = client.get("/no_route");
        let res4 = client.get("shouldn't be return *");
        let res5 = client.get("//gsaj");
        let res6 = client.get("");
        let res7 = client.get("/user/23");
        assert_eq!(res1, "HTTP/1.1 200 OK\r\n\r\npath request");
        assert_eq!(res2, "HTTP/1.1 200 OK\r\n\r\nhi good morning");
        assert_eq!(res3, "HTTP/1.1 404 Not Found\r\n\r\n");
        assert_eq!(res4, "HTTP/1.1 400 Bad Request\r\n\r\n");
        assert_eq!(res5, "HTTP/1.1 400 Bad Request\r\n\r\n");
        assert_eq!(res6, "HTTP/1.1 400 Bad Request\r\n\r\n");
        assert_eq!(res7, "HTTP/1.1 200 OK\r\n\r\nuser id is 23");
    }

    // #[test]
    // fn server_setup_test() {
    //     swish2().bish()
    // }

    #[derive(Debug, Deserialize, Serialize)]
    struct Sample {
        code: u16,
        data: String,
    }

    fn path_handler(req: Request) -> Box<dyn Body> {
        Box::new(Json(Sample {
            code: 200,
            data: "path request".to_string(),
        }))
    }

    fn greet_handler(req: Request) -> Box<dyn Body> {
        Box::new(Json(Sample {
            code: 200,
            data: "hi good morning".to_string(),
        }))
    }

    fn user_id_handler(req: Request) -> Box<dyn Body> {
        Box::new(Json(Sample {
            code: 200,
            data: format!("{}{}", "user id is ".to_string(), req.param),
        }))
    }

    fn swish2() -> Swish {
        let mut swish = Swish::new();
        swish.swish("/path", "GET", path_handler);
        swish.swish("/greet", "GET", greet_handler);
        swish.swish("/user/:id", "GET", user_id_handler);
        swish
    }
}
