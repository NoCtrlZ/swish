mod client;
mod entities;
mod error;
mod global;
mod http;
mod matcher;
mod request;
mod response;
mod router;
mod swish;

use crate::client::{request, Client};
use crate::request::Request;
use crate::response::Response;
use crate::swish::Swish;

extern crate regex;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn register_routing_test() {
        let mut swish = Swish::new();
        swish.swish("/=?", "GET", path_handler);
    }

    #[test]
    fn simple_path_test() {
        let mut client = Client::new(swish2());
        let res1 = client.get("/path");
        let res2 = client.get("/greet");
        assert_eq!(res1, "HTTP/1.1 200 OK\r\n\r\npath request");
        assert_eq!(res2, "HTTP/1.1 200 OK\r\n\r\nhi good morning");
    }

    #[test]
    fn not_fount_test() {
        let mut client = Client::new(swish2());
        let res = client.get("/no_route");
        assert_eq!(res, "HTTP/1.1 404 Not Found\r\n\r\n");
    }

    #[test]
    fn invalid_path_test() {
        let mut client = Client::new(swish2());
        let res1 = client.get("shouldn't be return *");
        let res2 = client.get("//gsaj");
        let res3 = client.get("");
        assert_eq!(res1, "HTTP/1.1 400 Bad Request\r\n\r\n");
        assert_eq!(res2, "HTTP/1.1 400 Bad Request\r\n\r\n");
        assert_eq!(res3, "HTTP/1.1 400 Bad Request\r\n\r\n");
    }

    #[test]
    fn dynamic_route_test() {
        let mut client = Client::new(swish2());
        let res1 = client.get("/user/23");
        assert_eq!(res1, "HTTP/1.1 200 OK\r\n\r\nuser id is 23");
    }

    #[test]
    fn server_setup_test() {
        swish2().bish()
    }

    fn path_handler(req: Request) -> Response {
        Response {
            status: 200,
            body: "path request".to_string(),
        }
    }

    fn greet_handler(req: Request) -> Response {
        Response {
            status: 200,
            body: "hi good morning".to_string(),
        }
    }

    fn user_id_handler(req: Request) -> Response {
        Response {
            status: 200,
            body: format!("{}{}", "user id is ".to_string(), req.param),
        }
    }

    fn swish2() -> Swish {
        let mut swish = Swish::new();
        swish.swish("/path", "GET", path_handler);
        swish.swish("/greet", "GET", greet_handler);
        swish.swish("/user/:id", "GET", user_id_handler);
        swish
    }
}
