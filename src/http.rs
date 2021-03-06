use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum StatusCode {
    Ok,
    BadRequest,
    Unauthorized,
    NotFound,
    InternalServerError,
}

impl StatusCode {
    pub fn get_code_number(&self) -> u16 {
        match self {
            &StatusCode::Ok => 200,
            &StatusCode::BadRequest => 400,
            &StatusCode::Unauthorized => 401,
            &StatusCode::NotFound => 404,
            &StatusCode::InternalServerError => 500,
        }
    }

    pub fn get_status_msg(&self) -> String {
        match self {
            &StatusCode::Ok => "OK".to_string(),
            &StatusCode::BadRequest => "Bad Request".to_string(),
            &StatusCode::Unauthorized => "Unauthorized".to_string(),
            &StatusCode::NotFound => "Not Found".to_string(),
            &StatusCode::InternalServerError => "Internal Server Error".to_string(),
        }
    }

    pub fn get_response_prefix(&self) -> String {
        format!(
            "{} {}",
            self.get_code_number().to_string(),
            self.get_status_msg()
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Method {
    GET,
    POST,
    OTHER,
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
        assert_eq!(StatusCode::Ok.get_response_prefix(), "200 OK");
        assert_eq!(
            StatusCode::BadRequest.get_response_prefix(),
            "400 Bad Request"
        );
        assert_eq!(StatusCode::NotFound.get_response_prefix(), "404 Not Found");
        assert_eq!(
            StatusCode::InternalServerError.get_response_prefix(),
            "500 Internal Server Error"
        );
    }
}
