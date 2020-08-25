use crate::body::Body;
use crate::header::Header;

use std::io::prelude::*;
use std::net::TcpStream;

pub struct Response {
    pub header: Header,
    pub body: Box<dyn Body>,
}

impl Response {
    pub fn send(&self, stream: &mut TcpStream) {
        let body = self.body.contents();
        let contents = format!(" {}\r\n{}", self.header.get_contents(body.len()), body);
        stream
            .write(contents.as_bytes())
            .expect("fail to write bytes");
        stream.flush().expect("fail to flush stream");
    }
}
