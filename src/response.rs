use crate::global::HTTP_VERSION;
use crate::http::get_status_code;
use crate::request::Request;
use crate::router::Handler;
use crate::types::Body;

use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub ctype: String,
    pub header: String,
    pub body: String,
}

impl Response {
    pub fn compile(&self) -> String {
        let mut response_data = HTTP_VERSION.to_string();
        let status_and_body = format!(
            " {}\r\n{}\r\n\r\n{}",
            get_status_code(self.status_code).compile(),
            self.get_basic_header(self.body.len()),
            self.body
        );
        response_data.push_str(&status_and_body);
        response_data
    }

    pub fn set_header(&mut self, header: &str) {
        self.header = header.to_string()
    }

    fn get_basic_header(&self, body_length: usize) -> String {
        format!("{}\r\nContent-Length: {}", self.header, body_length)
    }
}

pub fn write(contents: &str, stream: &mut TcpStream) {
    stream
        .write(contents.as_bytes())
        .expect("fail to write bytes");
    stream.flush().expect("fail to flush stream");
}
