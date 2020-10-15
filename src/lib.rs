mod body;
mod config;
mod cors;
mod header;
mod http;
mod request;
mod response;
mod router;
mod swish;
mod validater;

pub use crate::body::{Body, Json};
pub use crate::cors::{allow_everything, Cors};
pub use crate::request::Request;
pub use crate::swish::Swish;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    #[test]
    // fn start() {
    //     swish_swish().bish();
    // }

    fn defined_like_this() -> Cors {
        Cors {
            access_control_allow_origin: Some(["nothing".to_string()].to_vec()),
            ..Default::default()
        }
    }

    fn swish_swish() -> Swish {
        let mut swish = Swish::new();
        swish.get("/path", path_handler);
        swish.get("/user/:id", user_id_handler);
        swish.post("/user/register", user_register_handler);
        swish.set_cors_as(defined_like_this());
        swish.set_address("0.0.0.0");
        swish
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
        let param = match &req.param {
            Some(p) => p,
            None => "",
        };
        Box::new(Json(Sample {
            code: 200,
            data: format!("{}{}", "user id is ".to_string(), param),
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
