use crate::request::Request;
use crate::response::response;
use crate::swish::Swish;

use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

const PREFIX: &str = "HTTP/1.1\r\nHost: localhost:5862\r\nUser-Agent: curl/7.64.1\r\nAccept: */*";

mod method {
    pub const GET: &str = "GET ";
    pub const POST: &str = "POST ";
}

pub struct Client {
    pub server: Swish,
}

impl Client {
    pub fn new(swish: Swish) -> Client {
        Client { server: swish }
    }

    pub fn get(&mut self, path: &str) -> String {
        let mut req = Request {
            method: "GET".to_string(),
            path: path.to_string(),
            param: "".to_string(),
        };
        let handler = self.server.search(&mut req);
        let res = response(handler, req);
        res.compile()
    }
}

pub fn request(endpoint: &str) -> String {
    let mut stream = TcpStream::connect(endpoint).expect("fail to connect with endpoint");
    let mut contents = method::GET.to_string();
    contents.push_str(&format!("{}{}", "/", " "));
    contents.push_str(PREFIX);
    contents.push_str(&format!("{}{}", "\r\n\r\n", "body"));
    stream
        .write(&contents.as_bytes())
        .expect("fail to write data to stream");
    let mut buffer = [0; 8192];
    stream.read_exact(&mut buffer);
    String::from_utf8_lossy(&buffer[..])
        .trim_matches(char::from(0))
        .to_string()
}
