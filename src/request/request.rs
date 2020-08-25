use std::collections::HashMap;

use crate::http::Method;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub header: Header,
    pub body: String,
    pub param: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Header {
    pub prefix: String,
    pub elements: HashMap<String, String>,
}

impl Request {
    pub fn is_valid(&self) -> bool {
        self.method != Method::OTHER && self.path != ""
    }

    pub fn set_param(&mut self, param: &str) {
        self.param = Some(param.to_string());
    }

    pub fn get_param(&self) -> &Option<String> {
        &self.param
    }
}
