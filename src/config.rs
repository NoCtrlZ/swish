pub static HTTP_VERSION: &'static str = "HTTP/1.1";
pub static DEFALT_CHAR_TYPE: &'static str = "UTF-8";
pub static DEFAULT_HOST: &'static str = "127.0.0.1";
pub static DEFAULT_PORT: u16 = 3000;

#[derive(Debug, Clone)]
pub struct Config {
    http_version: String,
    char_type: String,
    address: String,
    port: u16,
}

impl Config {
    pub fn new() -> Config {
        Config {
            http_version: HTTP_VERSION.to_string(),
            char_type: DEFALT_CHAR_TYPE.to_string(),
            address: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
        }
    }

    pub fn get_version(&self) -> String {
        self.http_version.to_string()
    }

    pub fn get_charset(&self) -> String {
        format!("charset={}", self.char_type)
    }

    pub fn get_address(&self) -> String {
        self.address.to_string()
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_origin(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }

    pub fn set_char_type(&mut self, char_type: &str) {
        self.char_type = char_type.to_string();
    }

    pub fn set_address(&mut self, address: &str) {
        self.address = address.to_string();
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }
}
