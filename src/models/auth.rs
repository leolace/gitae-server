use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthClaims {
    pub user_id: i32,
    pub exp: i64 
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthPayload {
    pub token: String
}
