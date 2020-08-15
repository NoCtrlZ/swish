use crate::config::{HeaderConfig, HTTP_VERSION};
use crate::http::get_response_status_msg;
use crate::http::StatusCode;
use crate::request::Request;
use crate::router::Handler;
use crate::types::Body;

use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    pub status_code: StatusCode,
    pub ctype: String,
    pub header: String,
    pub body: String,
    pub header_conf: HeaderConfig,
}

impl Response {
    pub fn compile(&self) -> String {
        let mut response_data = self.header_conf.get_version();
        let status_and_body = format!(
            " {}\r\n{}\r\n\r\n{}",
            get_response_status_msg(self.status_code.clone()),
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
