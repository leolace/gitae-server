use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct JwtPayload {
    pub token: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserId {
    pub id: i32
}
