use std::net::TcpStream;

use crate::entities::convert_buffer_to_string;
use crate::http::{get_method, Method};

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub header: String,
    pub body: String,
    pub param: String,
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
    let contents = devide_into_contents(&header);
    let (method, path) = get_method_and_path(&contents[0]);
    Request {
        method: method,
        path: path,
        header: contents[1].clone(),
        body: body,
        param: "".to_string(),
    }
}

fn devide_header_and_body(req: &str) -> (String, String) {
    let components: Vec<String> = req.split("\r\n\r\n").map(|s| s.to_string()).collect();
    if components.len() < 2 {
        panic!("invalid request: body doesn't exist")
    }
    // println!("{:?}", components);
    (components[0].clone(), components[1].clone())
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
