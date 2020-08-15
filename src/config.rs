pub static HTTP_VERSION: &'static str = "HTTP/1.1";
pub static DEFALT_CHAR_TYPE: &'static str = "UTF-8";

#[derive(Debug, Clone)]
pub struct HeaderConfig {
    http_version: String,
    char_type: String,
}

impl HeaderConfig {
    pub fn new() -> HeaderConfig {
        HeaderConfig {
            http_version: HTTP_VERSION.to_string(),
            char_type: DEFALT_CHAR_TYPE.to_string(),
        }
    }

    pub fn get_version(&self) -> String {
        self.http_version.to_string()
    }

    pub fn get_charset(&self) -> String {
        format!("charset={}", self.char_type)
    }

    pub fn set_char_type(&mut self, char_type: &str) {
        self.char_type = char_type.to_string();
    }
}
