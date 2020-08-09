use std::io::prelude::*;
use std::net::TcpStream;
use crate::request::Request;

pub fn parse(stream: &mut TcpStream) -> Request {
    let raw_data = convert_buffer_to_string(stream);
    println!("{:?}", raw_data);
    let request = convert_string_to_request(&raw_data);
    println!("{:?}", request);
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

fn convert_buffer_to_string(stream: &mut TcpStream) -> String {
    // content type length limitation is 1000
    let mut buffer = [0; 1000];
    stream.read(&mut buffer).expect("fail to read buffer from stream");
    // println!("{:?}", &buffer[..]);
    buffer_to_string(&buffer[..])
}

fn buffer_to_string(buffer: &[u8]) -> String {
    String::from_utf8_lossy(&buffer[..])
    .trim_matches(char::from(0))
    .to_string()
}
