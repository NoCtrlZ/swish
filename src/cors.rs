use crate::http::Method;
use crate::request::Request;

#[derive(Default)]
pub struct Cors {
    pub access_control_allow_origin: Option<Vec<String>>,
    pub access_control_allow_headers: Option<Vec<String>>,
    pub access_control_allow_methods: Option<Vec<Method>>,
    pub access_control_allow_credential: Option<bool>,
}

impl Cors {
    pub fn validate_request(&self, req: &Request) -> (bool, String) {
        match &self.access_control_allow_origin {
            Some(origin) => {
                let (is_allowed_origin, msg) = self.validate_origin(&req, origin);
                if !is_allowed_origin {
                    return (is_allowed_origin, msg);
                }
            }
            None => (),
        }
        match &self.access_control_allow_headers {
            Some(headers) => {
                let (is_allowed_headers, msg) = self.validate_headers(&req, headers);
                if !is_allowed_headers {
                    return (is_allowed_headers, msg);
                }
            }
            None => (),
        }
        match &self.access_control_allow_methods {
            Some(methods) => {
                let (is_allowed_methods, msg) = self.validate_methods(&req, methods);
                if !is_allowed_methods {
                    return (is_allowed_methods, msg);
                }
            }
            None => (),
        }
        match &self.access_control_allow_credential {
            Some(credential) => {
                let (is_allowed_credential, msg) = self.validate_credential(&req, credential);
                if !is_allowed_credential {
                    return (is_allowed_credential, msg);
                }
            }
            None => (),
        }
        (true, "nothing has problem".to_string())
    }

    // This should be macro
    fn validate_origin(&self, req: &Request, origins: &Vec<String>) -> (bool, String) {
        match req.header.elements.get("Host") {
            Some(origin) => {
                for allowed_origin in origins {
                    if origin == allowed_origin {
                        return (true, "ok".to_string());
                    }
                }
                return (false, "origin is not allowed".to_string());
            }
            None => return (false, "invalid shaddy request".to_string()),
        }
    }

    fn validate_headers(&self, req: &Request, headers: &Vec<String>) -> (bool, String) {
        for (header, _) in &req.header.elements {
            if !headers.contains(&header) {
                return (false, "header is not allowed".to_string());
            }
        }
        (true, "ok".to_string())
    }

    fn validate_methods(&self, req: &Request, methods: &Vec<Method>) -> (bool, String) {
        for allowed_method in methods {
            if &req.method == allowed_method {
                return (true, "ok".to_string());
            }
        }
        (false, "method is not allowed".to_string())
    }

    fn validate_credential(&self, req: &Request, credential: &bool) -> (bool, String) {
        (true, "ok".to_string())
    }
}

pub fn allow_everything() -> Cors {
    Cors {
        access_control_allow_origin: None,
        access_control_allow_headers: None,
        access_control_allow_methods: None,
        access_control_allow_credential: None,
    }
}
