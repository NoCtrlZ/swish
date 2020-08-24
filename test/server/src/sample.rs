use swish_swish::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Sample {
    code: u16,
    data: String,
}

pub fn path_handler(_: &Request) -> Box<dyn Body> {
    Box::new(Json(Sample {
        code: 200,
        data: "path request".to_string(),
    }))
}

pub fn user_id_handler(req: &Request) -> Box<dyn Body> {
    Box::new(Json(Sample {
        code: 200,
        data: format!("{}{}", "user id is ".to_string(), req.param),
    }))
}

pub fn user_register_handler(req: &Request) -> Box<dyn Body> {
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