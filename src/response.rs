use crate::router::Handler;
use crate::request::Request;

use std::net::TcpStream;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Response {
    pub status: String,
    pub body: String,
}

impl Response {
    pub fn compile(self) -> String {
        format!("status: {} body: {}", self.status, self.body)
    }
}

pub fn response(handler: Handler, req: Request) -> Response {
    handler(req)
}

pub fn write(contents: &str, stream: &mut TcpStream) {
    stream.write(contents.as_bytes()).expect("fail to write bytes");
    stream.flush().expect("fail to flush stream");
}
