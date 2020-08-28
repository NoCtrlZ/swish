use crate::http::Method;
use crate::request::Request;
use crate::validater::ReqError;

#[derive(Default)]
pub struct Cors {
    pub access_control_allow_origin: Option<Vec<String>>,
    pub access_control_allow_headers: Option<Vec<String>>,
    pub access_control_allow_methods: Option<Vec<Method>>,
    pub access_control_allow_credential: Option<bool>,
}

impl Cors {
    pub fn validate_request(&self, req: &Request) -> (bool, ReqError) {
        match &self.access_control_allow_origin {
            Some(origin) => {
                let (is_allowed_origin, msg) = self.validate_origin(&req, origin);
                if !is_allowed_origin {
                    return (is_allowed_origin, ReqError::IsNotAllowedOrigin);
                }
            }
            None => (),
        }
        match &self.access_control_allow_headers {
            Some(headers) => {
                let (is_allowed_headers, msg) = self.validate_headers(&req, headers);
                if !is_allowed_headers {
                    return (is_allowed_headers, ReqError::IsNotAllowedHeader);
                }
            }
            None => (),
        }
        match &self.access_control_allow_methods {
            Some(methods) => {
                let (is_allowed_methods, msg) = self.validate_methods(&req, methods);
                if !is_allowed_methods {
                    return (is_allowed_methods, ReqError::IsNotAllowedMethod);
                }
            }
            None => (),
        }
        match &self.access_control_allow_credential {
            Some(credential) => {
                let (is_allowed_credential, msg) = self.validate_credential(&req, credential);
                if !is_allowed_credential {
                    return (is_allowed_credential, ReqError::IsNotAllowedCredential);
                }
            }
            None => (),
        }
        (true, ReqError::Empty)
    }

    // This should be macro
    fn validate_origin(&self, req: &Request, origins: &Vec<String>) -> (bool, ReqError) {
        match req.header.elements.get("Host") {
            Some(origin) => {
                for allowed_origin in origins {
                    if origin == allowed_origin {
                        return (true, ReqError::Empty);
                    }
                }
                return (false, ReqError::IsNotAllowedOrigin);
            }
            None => (false, ReqError::IsInvalid),
        }
    }

    fn validate_headers(&self, req: &Request, headers: &Vec<String>) -> (bool, ReqError) {
        for (header, _) in &req.header.elements {
            if !headers.contains(&header) {
                return (false, ReqError::IsNotAllowedHeader);
            }
        }
        (true, ReqError::Empty)
    }

    fn validate_methods(&self, req: &Request, methods: &Vec<Method>) -> (bool, ReqError) {
        for allowed_method in methods {
            if &req.method == allowed_method {
                return (true, ReqError::Empty);
            }
        }
        (false, ReqError::IsNotAllowedMethod)
    }

    fn validate_credential(&self, req: &Request, credential: &bool) -> (bool, ReqError) {
        (true, ReqError::Empty)
    }
}

pub fn allow_everything() -> Cors {
    Cors {
        ..Default::default()
    }
}
