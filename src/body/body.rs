use crate::http::StatusCode;

pub trait Body {
    fn status(&self) -> StatusCode;
    fn content_type(&self) -> String;
    fn contents(&self) -> String;
}
