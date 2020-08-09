use crate::router::Route;
use crate::request::Request;

pub fn match_with(req: &Request, route: &Route) -> bool {
    route.method == req.method && route.path == req.path
}
