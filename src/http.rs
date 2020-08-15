use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum StatusCode {
    Ok,
    BadRequest,
    NotFound,
    InternalServerError,
}

impl StatusCode {
    pub fn get_code_number(&self) -> u16 {
        match self {
            &StatusCode::Ok => 200,
            &StatusCode::BadRequest => 400,
            &StatusCode::NotFound => 404,
            &StatusCode::InternalServerError => 500,
            _ => 500,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    OTHER,
}

pub fn get_response_status_msg(code: StatusCode) -> String {
    match code {
        StatusCode::Ok => "200 OK".to_string(),
        StatusCode::BadRequest => "400 Bad Request".to_string(),
        StatusCode::NotFound => "404 Not Found".to_string(),
        StatusCode::InternalServerError => "500 Internal Server Error".to_string(),
        _ => "500 Internal Server Error".to_string(),
    }
}

pub fn get_method(req_method: &str) -> Method {
    match req_method {
        "GET" => Method::GET,
        "POST" => Method::POST,
        _ => Method::OTHER,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_status_msg_test() {
        let msg1 = get_response_status_msg(StatusCode::Ok);
        let msg2 = get_response_status_msg(StatusCode::BadRequest);
        let msg3 = get_response_status_msg(StatusCode::NotFound);
        let msg4 = get_response_status_msg(StatusCode::InternalServerError);
        assert_eq!(msg1, "OK");
        assert_eq!(msg2, "Bad Request");
        assert_eq!(msg3, "Not Found");
        assert_eq!(msg4, "Internal Server Error");
    }
}
