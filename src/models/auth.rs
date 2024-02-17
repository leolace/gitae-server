use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthClaims {
    pub user_id: Uuid,
    pub exp: i64 
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthPayload {
    pub token: String
}
