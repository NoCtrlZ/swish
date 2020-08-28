use regex::Regex;

pub fn split_slash(text: &str) -> Vec<String> {
    let mut pathes: Vec<String> = text.split("/").map(|s| s.to_string()).collect();
    pathes.drain(0..1);
    pathes
}

pub fn is_route_url(url: &str) -> bool {
    let url_regex = Regex::new(r"^/[\w]+(/[\w -.!:()])*").expect("fail to new regex");
    url_regex.is_match(&url)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_slash_test() {
        let res1 = split_slash("/path");
        let res2 = split_slash("/user/23");
        let res3 = split_slash("/user/:id");
        assert_eq!(res1, ["path"]);
        assert_eq!(res2, ["user", "23"]);
        assert_eq!(res3, ["user", ":id"]);
    }

    #[test]
    fn route_url_validation_test() {
        let res1 = is_route_url("/test");
        let res2 = is_route_url("//test");
        let res3 = is_route_url("test");
        let res4 = is_route_url("/h!.()-_:");
        let res5 = is_route_url("/=?");
        let res6 = is_route_url("/");
        let res7 = is_route_url("");
        assert_eq!(res1, true);
        assert_eq!(res2, false);
        assert_eq!(res3, false);
        assert_eq!(res4, true);
        assert_eq!(res5, false);
        assert_eq!(res6, false);
        assert_eq!(res7, false);
    }
}
