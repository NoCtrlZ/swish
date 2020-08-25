pub static DEFAULT_HOST: &'static str = "127.0.0.1";
pub static DEFAULT_PORT: u16 = 3000;

#[derive(Debug, Clone)]
pub struct Config {
    address: String,
    port: u16,
}

impl Config {
    pub fn new() -> Config {
        Config {
            address: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
        }
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

    pub fn set_address(&mut self, address: &str) {
        self.address = address.to_string();
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }
}
