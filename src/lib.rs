mod matcher;
mod router;
mod swish;
mod parser;
mod request;
mod client;
mod error;
mod entities;
mod response;

use crate::swish::Swish;
use crate::client::Client;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_path_test() {
        let mut client = Client::new(swish2());
        let res = client.get("/path");
        assert_eq!(res, "path request");
    }

    #[test]
    fn not_fount_test() {
        let mut client = Client::new(swish2());
        let res = client.get("/no_route");
        assert_eq!(res, "/no_route is not found");
    }

    #[test]
    fn invalid_path_test() {
        let mut client = Client::new(swish2());
        let res = client.get("shouldn't be return *");
    }

    #[test]
    fn server_setup_test() {
        swish2().bish()
    }

    fn path_handler(url: &str) -> String {
        "path request".to_string()
    }

    fn swish2() -> Swish {
        let mut swish = Swish::new();
        swish.swish("/path", "GET", path_handler);
        swish
    }
}
