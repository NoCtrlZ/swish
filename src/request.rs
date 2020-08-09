use std::net::TcpStream;

use crate::error::ReqErr;
use crate::entities::{convert_buffer_to_string, is_request_url};

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
}

pub fn parse(stream: &mut TcpStream) -> Result<Request, ReqErr> {
    let raw_data = convert_buffer_to_string(stream);
    // println!("{:?}", raw_data);
    convert_string_to_request(&raw_data)
}

fn convert_string_to_request(text: &str) -> Result<Request, ReqErr> {
    let mut components = text.split_whitespace();
    let method = match components.nth(0) {
        Some(e) => e,
        None => "",
    };
    let path = match components.nth(0) {
        Some(e) => e,
        None => "",
    };
    if method == "" || !is_request_url(path) {
        return Err(ReqErr::new("invalid request"));
    } else {
        return Ok(Request {
            method: method.to_string(),
            path: path.to_string(),
        });
    }
}
