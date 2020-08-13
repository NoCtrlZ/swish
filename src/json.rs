use crate::types::Body;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Json<T>(pub T);
