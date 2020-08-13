use crate::global::HTTP_VERSION;
use crate::http::get_status_code;
use crate::request::Request;
use crate::router::Handler;
use crate::types::Body;

use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn compile(self) -> String {
        let mut response_data = HTTP_VERSION.to_string();
        let status_and_body = format!(
            " {}\r\n\r\n{}",
            get_status_code(self.status).compile(),
            self.body
        );
        response_data.push_str(&status_and_body);
        response_data
    }
}

pub fn write(contents: &str, stream: &mut TcpStream) {
    stream
        .write(contents.as_bytes())
        .expect("fail to write bytes");
    stream.flush().expect("fail to flush stream");
}
