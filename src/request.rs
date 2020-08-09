use std::net::TcpStream;
use crate::entities::convert_buffer_to_string;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
}

pub fn parse(stream: &mut TcpStream) -> Request {
    let raw_data = convert_buffer_to_string(stream);
    // println!("{:?}", raw_data);
    let request = convert_string_to_request(&raw_data);
    // println!("{:?}", request);
    request
}


fn convert_string_to_request(text: &str) -> Request {
    let mut components = text.split_whitespace();
    let method = match components.nth(0) {
        Some(e) => e,
        None => "GET",
    };
    let path = match components.nth(0) {
        Some(e) => e,
        None => "/",
    };
    Request {
        method: method.to_string(),
        path: path.to_string(),
    }
}
