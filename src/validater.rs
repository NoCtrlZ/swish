use crate::cors::Cors;
use crate::error::ReqError;
use crate::request::Request;

#[derive(Default)]
pub struct Validater {
    cors: Option<Cors>,
}

impl Validater {
    pub fn set_cors(&mut self, cors: Cors) {
        self.cors = Some(cors)
    }

    pub fn validate_request(&self, req: &Request) -> (bool, ReqError) {
        match &self.cors {
            Some(cors) => {
                return cors.validate_request(&req);
            }
            None => (true, ReqError::Empty),
        }
    }
}
