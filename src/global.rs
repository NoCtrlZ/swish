pub static HTTP_VERSION: &'static str = "HTTP/1.1";
pub static DEFALT_CHAR_TYPE: &'static str = "UTF-8";

pub struct Config {
    version: String,
    char_type: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            version: HTTP_VERSION.to_string(),
            char_type: DEFALT_CHAR_TYPE.to_string(),
        }
    }

    pub fn get_version(self) -> String {
        self.version.to_string()
    }

    pub fn get_charset(&self) -> String {
        format!("charset={}", self.char_type)
    }

    pub fn set_char_type(&mut self, char_type: &str) {
        self.char_type = char_type.to_string();
    }
}
