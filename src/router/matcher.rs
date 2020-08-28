use crate::request::Request;
use crate::router::entities::split_slash;
use crate::router::Route;

pub fn match_with(req: &mut Request, route: &Route) -> bool {
    if route.method == req.method {
        let pathes = split_slash(&req.path);
        let routes = split_slash(&route.path);
        if pathes.len() == routes.len() {
            if is_static_route(routes.clone()) {
                for n in 0..pathes.len() {
                    if pathes[n] == routes[n] {
                        continue;
                    } else {
                        return false;
                    }
                }
                true
            } else {
                for n in 0..pathes.len() {
                    if routes[n].chars().next().expect("fail to get next char") == ':' {
                        req.set_param(&pathes[n]);
                        continue;
                    } else if pathes[n] == routes[n] {
                        continue;
                    } else {
                        return false;
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

fn is_static_route(routes: Vec<String>) -> bool {
    for path in &routes {
        if path.chars().next().expect("fail to get next char") == ':' {
            return false;
        } else {
            continue;
        }
    }
    true
}
