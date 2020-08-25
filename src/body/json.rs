use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::body::Body;
use crate::http::StatusCode;

#[derive(Debug, Deserialize, Serialize)]
pub struct Json<T>(pub T);

impl<T: Serialize> Body for Json<T> {
    fn content_type(&self) -> String {
        "application/json".to_string()
    }

    fn status(&self) -> StatusCode {
        StatusCode::Ok
    }

    fn body(&self) -> String {
        json!(self.0).to_string()
    }
}
