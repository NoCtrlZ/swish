use crate::config::Config;
use crate::http::StatusCode;
// todo should be in Response body field
use crate::body::Body;
use crate::header::Header;

use std::io::prelude::*;
use std::net::TcpStream;

pub struct Response {
    pub header: Header,
    pub body: Box<dyn Body>,
}

impl Response {
    pub fn compile(&self) -> String {
        let body = self.body.contents();
        format!(" {}\r\n{}", self.header.get_contents(body.len()), body)
    }
}

pub fn write(contents: &str, stream: &mut TcpStream) {
    stream
        .write(contents.as_bytes())
        .expect("fail to write bytes");
    stream.flush().expect("fail to flush stream");
}
