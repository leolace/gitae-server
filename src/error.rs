use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HttpError {
    #[serde(skip)]
    pub code: StatusCode,
    pub error: String,
}

impl HttpError {
    pub fn new(code: StatusCode, error: &str) -> HttpError {
        HttpError {
            code,
            error: String::from(error),
        }
    }
}
