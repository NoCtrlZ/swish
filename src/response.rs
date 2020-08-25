use crate::config::Config;
use crate::http::StatusCode;
// todo should be in Response body field
use crate::body::Body;

use std::io::prelude::*;
use std::net::TcpStream;

pub struct Response {
    pub status_code: StatusCode,
    pub ctype: String,
    pub header: String,
    pub body: Box<dyn Body>,
    pub header_conf: Config,
}

impl Response {
    pub fn compile(&self) -> String {
        let mut response_data = self.header_conf.get_version();
        let body = self.body.contents();
        let status_and_body = format!(
            " {}\r\n{}\r\n\r\n{}",
            self.status_code.get_response_prefix(),
            self.get_basic_header(body.len()),
            body
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
