mod matcher;
mod router;
mod swish;
mod request;
mod client;
mod error;
mod entities;
mod response;

use crate::swish::Swish;
use crate::client::Client;
use crate::response::Response;
use crate::request::Request;

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
        assert_eq!(res1, "status: 200 body: path request");
        assert_eq!(res2, "status: 200 body: hi good morning");
    }

    #[test]
    fn not_fount_test() {
        let mut client = Client::new(swish2());
        let res = client.get("/no_route");
        assert_eq!(res, "status: 404 body: /no_route was not found");
    }

    #[test]
    fn invalid_path_test() {
        let mut client = Client::new(swish2());
        let res1 = client.get("shouldn't be return *");
        let res2 = client.get("//gsaj");
        let res3 = client.get("");
        assert_eq!(res1, "status: 400 body: request is not valid");
        assert_eq!(res2, "status: 400 body: request is not valid");
        assert_eq!(res3, "status: 400 body: request is not valid");
    }

    #[test]
    fn dynamic_route_test() {
        let mut client = Client::new(swish2());
        let res1 = client.get("/user/23");
        assert_eq!(res1, "status: 200 body: user id is 23");
    }

    // #[test]
    // fn server_setup_test() {
    //     swish2().bish()
    // }

    fn path_handler(req: Request) -> Response {
        Response {
            status: "200".to_string(),
            body: "path request".to_string(),
        }
    }

    fn greet_handler(req: Request) -> Response {
        Response {
            status: "200".to_string(),
            body: "hi good morning".to_string(),
        }
    }

    fn user_id_handler(req: Request) -> Response {
        Response {
            status: "200".to_string(),
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
