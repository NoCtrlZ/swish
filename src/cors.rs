use crate::http::Method;

#[derive(Default)]
pub struct Cors {
    pub access_control_allow_origin: String,
    pub access_control_allow_headers: Vec<String>,
    pub access_control_allow_methods: Vec<Method>,
    pub access_control_allow_credentials: bool,
}
