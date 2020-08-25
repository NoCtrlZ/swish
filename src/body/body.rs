use crate::http::StatusCode;

pub trait Body {
    fn status(&self) -> StatusCode;
    fn content_type(&self) -> String;
    fn body(&self) -> String;
}
