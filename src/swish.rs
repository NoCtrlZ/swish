use crate::router;
use crate::matcher;

use crate::router::Router;

pub struct Swish {
    pub router: Router,
}

pub fn swish() -> Swish {
    Swish {
        router: Router {
            routes: Vec::new(),
        }
    }
}
