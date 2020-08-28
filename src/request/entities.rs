use std::io::prelude::*;
use std::net::TcpStream;

fn buffer_to_string(buffer: &[u8]) -> String {
    String::from_utf8_lossy(&buffer[..])
        .trim_matches(char::from(0))
        .to_string()
}

pub fn convert_buffer_to_string(stream: &mut TcpStream) -> String {
    // content type length limitation is 1000
    let mut buffer = [0; 1000];
    stream
        .read(&mut buffer)
        .expect("fail to read buffer from stream");
    // println!("{:?}", &buffer[..]);
    buffer_to_string(&buffer[..])
}
