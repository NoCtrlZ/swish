#[derive(Debug)]
pub struct ReqErr {
    pub msg: String
}

impl ReqErr {
    pub fn new(msg: &str) -> ReqErr {
        ReqErr {
            msg: msg.to_string(),
        }
    }
}

pub fn not_found(url: &str) -> String {
    format!("{}{}", url, " is not found")
}
