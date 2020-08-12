use std::net::TcpStream;

use crate::entities::convert_buffer_to_string;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub param: String,
}

impl Request {
    pub fn is_valid(&self) -> bool {
        self.method != "" && self.path != ""
    }

    pub fn set_param(&mut self, param: &str) {
        self.param = param.to_string();
    }

    pub fn get_param(self) -> String {
        self.param
    }
}

pub fn parse(stream: &mut TcpStream) -> Request {
    let raw_data = convert_buffer_to_string(stream);
    println!("{:?}", raw_data);
    convert_string_to_request(&raw_data)
}

fn convert_string_to_request(text: &str) -> Request {
    let mut components = text.split_whitespace();
    let method = match components.nth(0) {
        Some(e) => e,
        None => "",
    };
    let path = match components.nth(0) {
        Some(e) => e,
        None => "",
    };
    Request {
        method: method.to_string(),
        path: path.to_string(),
        param: "".to_string(),
    }
}
