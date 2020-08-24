extern crate swish_swish;

mod sample;

use swish_swish::*;
use sample::{path_handler, user_id_handler, user_register_handler};

fn defined_like_this() -> Cors {
    Cors {
        access_control_allow_origin: Some(["nothing"]),
        ..Default::default()
    }
}

fn swish_swish() -> Swish {
    let mut swish = Swish::new();
    swish.get("/path", path_handler);
    swish.get("/user/:id", user_id_handler);
    swish.post("/user/register", user_register_handler);
    swish.set_cors_as(allow_everything());
    swish.set_address("0.0.0.0");
    swish
}

fn main() {
    swish_swish().bish();
}
