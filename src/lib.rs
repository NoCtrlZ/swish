mod matcher;
mod router;
mod swish;
mod parser;
mod request;

use crate::swish::Swish;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matcher_test() {
        let matched = matcher::match_with("this is test");
        assert_eq!(matched, "this is test");
    }

    #[test]
    fn router_test() {
        let mut router = router::Router::new();
        router.register("/path", "GET", test_handler);
    }

    #[test]
    fn server_test() {
        let mut swish2 = swish2();
        swish2.bish();
        assert_eq!(swish2.router.routes.len(), 1);
    }

    fn test_handler(url: &str) -> String {
        url.to_string()
    }

    fn swish2() -> Swish {
        let mut swish = Swish::new();
        swish.swish("/path", "GET", test_handler);
        swish
    }
}
