mod matcher;
mod router;
mod swish;
mod parser;
mod request;
mod client;
mod error;

use crate::swish::Swish;
use crate::client::Client;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn router_test() {
        let mut router = router::Router::new();
        router.register("/path", "GET", test_handler);
    }

    #[test]
    fn server_test() {
        let mut swish2 = swish2();
        assert_eq!(swish2.router.routes.len(), 1);
    }

    #[test]
    fn client_test() {
        let mut client = Client::new(swish2());
        let res = client.get("/path");
        assert_eq!(res, "/path");
    }

    #[test]
    fn client_not_fount_test() {
        let mut client = Client::new(swish2());
        let res = client.get("/no_route");
        assert_eq!(res, "/no_route is not found");
    }

    // #[test]
    // fn server_start() {
    //     swish2().bish()
    // }

    fn test_handler(url: &str) -> String {
        url.to_string()
    }

    fn swish2() -> Swish {
        let mut swish = Swish::new();
        swish.swish("/path", "GET", test_handler);
        swish
    }
}
