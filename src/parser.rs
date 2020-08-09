use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub body: String,
}

pub fn parse(stream: &mut TcpStream) {
    let raw_data = convert(stream);
    println!("{:?}", raw_data);
}

fn convert(stream: &mut TcpStream) -> String {
    // content type length limitation is 5000
    let mut buffer = [0; 5000];
    stream.read(&mut buffer).expect("fail to read buffer from stream");
    println!("{:?}", &buffer[..]);
    String::from_utf8_lossy(&buffer[..])
        .trim_matches(char::from(0))
        .to_string()
}
