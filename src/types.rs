use crate::json::Json;

use serde::Serialize;
use serde_json::json;

pub trait Body {
    fn compile(&self) -> String;
    fn get_content_type(&self) -> String;
}

impl<T: Serialize> Body for Json<T> {
    fn compile(&self) -> String {
        json!(self.0).to_string()
    }

    fn get_content_type(&self) -> String {
        "application/json".to_string()
    }
}
