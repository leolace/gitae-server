use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Error {
    #[serde(skip)]
    pub code: StatusCode,
    pub error: &'static str,
}

impl Error {
    pub fn new(code: StatusCode, error: &'static str) -> Error {
        Error { code, error }
    }
}
