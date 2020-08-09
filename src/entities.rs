use regex::Regex;

pub fn buffer_to_string(buffer: &[u8]) -> String {
    String::from_utf8_lossy(&buffer[..])
    .trim_matches(char::from(0))
    .to_string()
}

pub fn is_route_url(url: &str) -> bool {
    let url_regex = Regex::new(r"^/[\w]+(/[\w -.!:()])*").unwrap();
    url_regex.is_match(&url)
}

pub fn is_request_url(url: &str) -> bool {
    let url_regex = Regex::new(r"^/[\w]+(/[\w -.!?=()])*").unwrap();
    url_regex.is_match(&url)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn route_url_validation_test() {
        let res1 = is_route_url("/test");
        let res2 = is_route_url("//test");
        let res3 = is_route_url("test");
        let res4 = is_route_url("/h!.()-_:");
        let res5 = is_route_url("/=?");
        let res6 = is_route_url("/");
        assert_eq!(res1, true);
        assert_eq!(res2, false);
        assert_eq!(res3, false);
        assert_eq!(res4, true);
        assert_eq!(res5, false);
        assert_eq!(res6, false);
    }

    #[test]
    fn request_url_validation_test() {
        let res1 = is_request_url("/test");
        let res2 = is_request_url("//test");
        let res3 = is_request_url("test");
        let res4 = is_request_url("/test?var=true");
        let res5 = is_request_url("/!.()-:l");
        let res6 = is_request_url("/");
        assert_eq!(res1, true);
        assert_eq!(res2, false);
        assert_eq!(res3, false);
        assert_eq!(res4, true);
        assert_eq!(res5, false);
        assert_eq!(res6, false);
    }
}
