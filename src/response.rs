use crate::router::Handler;
use crate::request::Request;

use std::net::{TcpStream, TcpListener};
use std::io::prelude::*;

pub fn response(handler: Handler, req: Request) -> String {
    handler(&req.path)
}

pub fn write(contents: &str, stream: &mut TcpStream) {
    stream.write(contents.as_bytes()).expect("fail to write bytes");
    stream.flush().expect("fail to flush stream");
}
