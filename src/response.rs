use crate::router::Handler;
use crate::request::Request;
use crate::global::HTTP_VERSION;

use std::net::TcpStream;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Response {
    pub status: String,
    pub body: String,
}

impl Response {
    pub fn compile(self) -> String {
        let mut response_data = HTTP_VERSION.to_string();
        let status_and_body = format!("status: {} body: {}", self.status, self.body);
        response_data.push_str(&status_and_body);
        response_data
    }
}

pub fn response(handler: Handler, req: Request) -> Response {
    handler(req)
}

pub fn write(contents: &str, stream: &mut TcpStream) {
    stream.write(contents.as_bytes()).expect("fail to write bytes");
    stream.flush().expect("fail to flush stream");
}
