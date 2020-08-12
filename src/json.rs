use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Json<T> {
    pub body: T,
}

impl<T: Serialize> Json<T> {
    pub fn compile(self) -> String {
        json!(self.body).to_string()
    }
}
