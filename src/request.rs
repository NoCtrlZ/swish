use std::collections::HashMap;
use std::net::TcpStream;

use crate::entities::convert_buffer_to_string;
use crate::http::{get_method, Method};

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub header: Header,
    pub body: String,
    pub param: String,
}

#[derive(Debug)]
pub struct Header {
    prefix: String,
    header: HashMap<String, String>,
}

impl Request {
    pub fn is_valid(&self) -> bool {
        self.method != Method::OTHER && self.path != ""
    }

    pub fn set_param(&mut self, param: &str) {
        self.param = param.to_string();
    }

    pub fn get_param(self) -> String {
        self.param
    }
}

pub fn parse(stream: &mut TcpStream) -> Request {
    let req = convert_buffer_to_string(stream);
    println!("{:?}", req);
    let (header, body) = devide_header_and_body(&req);
    let contents = devide_into_contents(&header.prefix);
    let (method, path) = get_method_and_path(&contents[0]);
    Request {
        method: method,
        path: path,
        header: header,
        body: body,
        param: "".to_string(),
    }
}

fn devide_header_and_body(req: &str) -> (Header, String) {
    let components: Vec<String> = req.split("\r\n\r\n").map(|s| s.to_string()).collect();
    if components.len() < 2 {
        panic!("invalid request: body doesn't exist")
    }
    // println!("{:?}", components);
    (
        convert_header_text_to_struct(&components[0]),
        components[1].clone(),
    )
}

fn convert_header_text_to_struct(header_text: &str) -> Header {
    let components: Vec<String> = header_text.split("\r\n").map(|s| s.to_string()).collect();
    let mut header = HashMap::new();
    for i in 1..components.len() {
        let mut elements = components[i].split_whitespace();
        let left = match elements.nth(0) {
            Some(e) => e.replace(":", ""),
            None => panic!("method can't be gotten"),
        };
        let right = match elements.nth(0) {
            Some(e) => e,
            None => "",
        };
        header.insert(left, right.to_string());
    }
    Header {
        prefix: components[0].clone(),
        header: header,
    }
}

fn devide_into_contents(req: &str) -> Vec<String> {
    let components: Vec<String> = req.split("\r\n").map(|s| s.to_string()).collect();
    if components.len() < 2 {
        panic!("invalid request: header or prefix doesn't exist")
    }
    components
}

fn get_method_and_path(req: &str) -> (Method, String) {
    let mut components = req.split_whitespace();

    let method = match components.nth(0) {
        Some(e) => get_method(e),
        None => panic!("method can't be gotten"),
    };
    let path = match components.nth(0) {
        Some(e) => e,
        None => "",
    };
    (method, path.to_string())
}
