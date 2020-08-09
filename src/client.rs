use crate::swish::{Swish};

pub struct Client {
    pub server: Swish
}

impl Client {
    pub fn new(swish: Swish) -> Client {
        Client {
            server: swish,
        }
    }
}
