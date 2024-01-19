use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignUp {
    #[serde(default)]
    pub username: String,

    #[serde(default)]
    pub email: String,

    #[serde(default)]
    pub password: String,
}
