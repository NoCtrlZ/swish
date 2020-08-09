mod matcher;
mod router;
mod swish;

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
        router.register("path", test_handler);
    }

    #[test]
    fn server_test() {
        let mut swish = Swish::new();
        swish.swish("path", test_handler);
        assert_eq!(swish.router.routes.len(), 1);
    }

    fn test_handler(url: &str) -> String {
        url.to_string()
    }
}
