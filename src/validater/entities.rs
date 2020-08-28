use regex::Regex;

pub fn is_request_url(url: &str) -> bool {
    let url_regex = Regex::new(r"^/[\w]+(/[\w -.!?=()])*").expect("fail to new regex");
    url_regex.is_match(&url)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn request_url_validation_test() {
        let res1 = is_request_url("/test");
        let res2 = is_request_url("//test");
        let res3 = is_request_url("test");
        let res4 = is_request_url("/test?var=true");
        let res5 = is_request_url("/!.()-:l");
        let res6 = is_request_url("/");
        let res7 = is_request_url("");
        assert_eq!(res1, true);
        assert_eq!(res2, false);
        assert_eq!(res3, false);
        assert_eq!(res4, true);
        assert_eq!(res5, false);
        assert_eq!(res6, false);
        assert_eq!(res7, false);
    }
}
