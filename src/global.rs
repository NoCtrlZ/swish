pub static HTTP_VERSION: &'static str = "HTTP/1.1";
pub static DEFALT_CONTENT_CYPE: &'static str = "application/json";
pub static DEFALT_CHAR_TYPE: &'static str = "UTF-8";

pub struct Config {
    version: String,
    content_type: String,
    char_type: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            version: HTTP_VERSION.to_string(),
            content_type: DEFALT_CONTENT_CYPE.to_string(),
            char_type: DEFALT_CHAR_TYPE.to_string(),
        }
    }

    pub fn get_version(self) -> String {
        self.version.to_string()
    }

    pub fn get_content_type(self) -> String {
        format!(
            "Content-Type: {}; charset={}",
            self.content_type, self.char_type
        )
    }

    pub fn set_content_type(&mut self, ctype: &str) {
        self.content_type = ctype.to_string();
    }

    pub fn set_char_type(&mut self, char_type: &str) {
        self.char_type = char_type.to_string();
    }
}
