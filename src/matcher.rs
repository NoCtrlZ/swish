use crate::router::Route;
use crate::request::Request;
use crate::entities::split_slash;

pub fn match_with(req: &Request, route: &Route) -> bool {
    if (route.method == req.method) {
        let pathes = split_slash(&req.path);
        let routes = split_slash(&route.path);
        if pathes.len() == routes.len() {
            if is_static_route(routes.clone()) {
                for n in 0..pathes.len() {
                    if pathes[n] == routes[n] {
                        continue;
                    } else {
                        return false
                    }
                }
                true
            } else {
                for n in 0..pathes.len() {
                    if routes[n].chars().next().expect("fail to get next char") == ':' {
                        continue;
                    } else if pathes[n] == routes[n] {
                        continue;
                    } else {
                        return false
                    }
                }
                true
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn is_static_route(routes :Vec<String>) -> bool {
    for path in &routes {
        if path.chars().next().expect("fail to get next char") == ':' {
            return false
        } else {
            continue;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn match_with_test() {
        let req = Request {
            method: "GET".to_string(),
            path: "/user/23".to_string(),
        };
        let route = Route {
            method: "GET".to_string(),
            path: "/user/:id".to_string(),
            handler: user_route_handler,
        };
        let res1 = match_with(&req, &route);
        assert_eq!(res1, true);
    }

    #[test]
    fn static_or_dynamic_test() {
        let pathes1 = ["hey".to_string(), "ether".to_string()];
        let pathes2 = [":hey".to_string(), "ether".to_string()];
        let res1 = is_static_route(pathes1.to_vec());
        let res2 = is_static_route(pathes2.to_vec());
        assert_eq!(res1, true);
        assert_eq!(res2, false);
    }

    #[test]
    fn split_slash_test() {
        let pathes = split_slash("/user/23");
        let routes = split_slash("/user/:id");
        let res1 = is_static_route(routes.clone());
        assert_eq!(pathes, ["user", "23"]);
        assert_eq!(routes, ["user", ":id"]);
        assert_eq!(res1, false);
    }

    fn user_route_handler(url: &str) -> String {
        "user path success".to_string()
    }
}
