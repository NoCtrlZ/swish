use std::collections::HashMap;

use crate::http::StatusCode;

pub static HTTP_VERSION: &'static str = "HTTP/1.1";
pub static DEFALT_CHAR_TYPE: &'static str = "UTF-8";

pub struct Header {
    pub status_code: StatusCode,
    pub ctype: String,
    pub http_version: String,
    pub char_type: String,
    pub headers: Option<HashMap<String, String>>,
}

impl Header {
    pub fn get_contents(&self, clength: usize) -> String {
        format!(
            "{}{}{}",
            self.get_prefix(),
            self.get_contents_type(),
            self.get_content_length(clength)
        )
    }

    fn get_prefix(&self) -> String {
        format!(
            "{} {}\r\n",
            self.http_version,
            self.status_code.get_response_prefix()
        )
    }

    fn get_contents_type(&self) -> String {
        format!("Content-Type: {}; {}\r\n", self.ctype, self.char_type)
    }

    fn get_content_length(&self, clength: usize) -> String {
        format!("Content-Length: {}\r\n", clength.to_string())
    }
}

pub fn compose_header(status_code: StatusCode, ctype: String) -> Header {
    Header {
        status_code: status_code,
        ctype: ctype,
        http_version: HTTP_VERSION.to_string(),
        char_type: DEFALT_CHAR_TYPE.to_string(),
        headers: None,
    }
}
