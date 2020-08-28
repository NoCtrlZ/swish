use std::collections::HashMap;
use std::net::TcpStream;

use crate::http::{get_method, Method};
use crate::request::entities::convert_buffer_to_string;
use crate::request::{Header, Request};

macro_rules! divide_pair {
    ($c: expr, $m: expr) => {{
        let mut elements = $c.split_whitespace();
        let mut pair: [String; 2] = ["".to_string(), "".to_string()];
        for i in 0..1 {
            pair[i] = elements.next().expect($m).to_string();
        }
        (pair[0].clone(), pair[1].clone())
    }};
}

pub fn parse(stream: &mut TcpStream) -> Request {
    let req = convert_buffer_to_string(stream);
    println!("{:?}", req);
    let (header, body) = devide_header_and_body(&req);
    let (method, path) = get_method_and_path(&header.prefix);
    Request {
        method: method,
        path: path,
        header: header,
        body: body,
        param: Default::default(),
    }
}

fn devide_header_and_body(req: &str) -> (Header, String) {
    let components: Vec<String> = req.split("\r\n\r\n").map(|s| s.to_string()).collect();
    if components.len() < 2 {
        panic!("invalid request: body doesn't exist")
    }
    (
        convert_header_text_to_struct(&components[0]),
        components[1].clone(),
    )
}

fn convert_header_text_to_struct(header_text: &str) -> Header {
    let components: Vec<String> = header_text.split("\r\n").map(|s| s.to_string()).collect();
    let mut header = HashMap::new();
    for i in 1..components.len() {
        let (left, right) = divide_pair!(components[i], "error happens when parsing header");
        header.insert(left.to_string(), right.to_string());
    }
    Header {
        prefix: components[0].clone(),
        elements: header,
    }
}

fn get_method_and_path(req: &str) -> (Method, String) {
    let (method, path) = divide_pair!(req, "error happens when parsing header prefix");
    (get_method(&method), path.to_string())
}
