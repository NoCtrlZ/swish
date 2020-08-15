mod config;
mod entities;
mod error;
mod http;
mod json;
mod matcher;
mod request;
mod response;
mod router;
mod swish;
mod types;

extern crate regex;

#[cfg(test)]
mod tests {
    use crate::json::Json;
    use crate::request::Request;
    use crate::swish::Swish;
    use crate::types::Body;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    // #[test]
    // fn setup_server() {
    //     swish_swish().bish()
    // }

    #[derive(Deserialize, Serialize)]
    struct Sample {
        code: u16,
        data: String,
    }

    fn swish_swish() -> Swish {
        let mut swish = Swish::new();
        swish.get("/path", path_handler);
        swish.get("/greet", greet_handler);
        swish.get("/user/:id", user_id_handler);
        swish.post("/user/register", user_register_handler);
        swish
    }

    fn path_handler(req: &Request) -> Box<dyn Body> {
        Box::new(Json(Sample {
            code: 200,
            data: "path request".to_string(),
        }))
    }

    fn greet_handler(req: &Request) -> Box<dyn Body> {
        Box::new(Json(Sample {
            code: 200,
            data: "hi good morning".to_string(),
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
}
