use crate::http::Method;
use crate::request::Request;

#[derive(Default)]
pub struct Cors {
    pub access_control_allow_origin: Option<String>,
    pub access_control_allow_headers: Option<Vec<String>>,
    pub access_control_allow_methods: Option<Vec<Method>>,
    pub access_control_allow_credentials: Option<bool>,
}

impl Cors {
    pub fn validate_request(&self, req: &Request) -> (bool, String) {
        if self.access_control_allow_origin.is_some() {
            let (isAllowedOrigin, msg) = self.validate_origin();
            if !isAllowedOrigin {
                return (isAllowedOrigin, msg);
            }
        }
        if self.access_control_allow_headers.is_some() {
            let (isAllowedOrigin, msg) = self.validate_headers();
            if !isAllowedOrigin {
                return (isAllowedOrigin, msg);
            }
        }
        if self.access_control_allow_methods.is_some() {
            let (isAllowedOrigin, msg) = self.validate_method();
            if !isAllowedOrigin {
                return (isAllowedOrigin, msg);
            }
        }
        if self.access_control_allow_credentials.is_some() {
            let (isAllowedOrigin, msg) = self.validate_credentials();
            if !isAllowedOrigin {
                return (isAllowedOrigin, msg);
            }
        }
        (true, "nothing has problem".to_string())
    }

    // This should be macro
    fn validate_origin(&self) -> (bool, String) {
        (true, "ok".to_string())
    }

    fn validate_headers(&self) -> (bool, String) {
        (true, "ok".to_string())
    }

    fn validate_method(&self) -> (bool, String) {
        (true, "ok".to_string())
    }

    fn validate_credentials(&self) -> (bool, String) {
        (true, "ok".to_string())
    }
}
