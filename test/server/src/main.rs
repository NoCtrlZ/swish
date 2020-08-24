extern crate swish_swish;

use swish_swish::*;
use serde::{Deserialize, Serialize};

fn cors() -> Cors {
    allow_everything()
}

fn swish_swish() -> Swish {
    let mut swish = Swish::new();
    swish.get("/path", path_handler);
    swish.get("/user/:id", user_id_handler);
    swish.post("/user/register", user_register_handler);
    swish.set(cors());
    swish
}

fn main() {
    swish_swish().bish();
    println!("Hello, world!");
}

#[derive(Deserialize, Serialize)]
struct Sample {
    code: u16,
    data: String,
}

fn path_handler(_: &Request) -> Box<dyn Body> {
    Box::new(Json(Sample {
        code: 200,
        data: "path request".to_string(),
    }))
}

fn user_id_handler(req: &Request) -> Box<dyn Body> {
    Box::new(Json(Sample {
        code: 200,
        data: format!("{}{}", "user id is ".to_string(), req.param),
    }))
}

fn user_register_handler(req: &Request) -> Box<dyn Body> {
    let sample: Sample =
        serde_json::from_str(&req.body).expect("fail to convert transaction to json");
    Box::new(Json(Sample {
        code: 200,
        data: format!(
            "success register id: {} msg: {}",
            sample.code.to_string(),
            sample.data
        ),
    }))
}
