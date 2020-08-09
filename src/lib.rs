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

extern crate regex;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_path_test() {
        let mut client = Client::new(swish2());
        let res1 = client.get("/path");
        let res2 = client.get("/greet");
        assert_eq!(res1, "path request");
        assert_eq!(res2, "hi good morning");
    }

    #[test]
    fn not_fount_test() {
        let mut client = Client::new(swish2());
        let res = client.get("/no_route");
        assert_eq!(res, "/no_route is not found");
    }

    // #[test]
    // fn invalid_path_test() {
    //     let mut client = Client::new(swish2());
    //     let res1 = client.get("shouldn't be return *");
    //     let res2 = client.get("//gsaj");
    //     let res3 = client.get("");
    //     assert_eq!(res1, "invalid request");
    //     assert_eq!(res2, "invalid request");
    //     assert_eq!(res3, "invalid request");
    // }

    // #[test]
    // fn server_setup_test() {
    //     swish2().bish()
    // }

    fn path_handler(url: &str) -> String {
        "path request".to_string()
    }

    fn greet_handler(url: &str) -> String {
        "hi good morning".to_string()
    }

    fn swish2() -> Swish {
        let mut swish = Swish::new();
        swish.swish("/path", "GET", path_handler);
        swish.swish("/greet", "GET", greet_handler);
        swish
    }
}
