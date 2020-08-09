use crate::router;
use crate::matcher;

use crate::router::{Router, Handler};

pub struct Swish {
    pub router: Router,
}

impl Swish {
    pub fn new() -> Swish {
        Swish {
            router: Router {
                routes: Vec::new(),
            }
        }
    }

    pub fn swish(&mut self, path: &str, handler: Handler) {
        self.router.register(path, handler)
    }
}
