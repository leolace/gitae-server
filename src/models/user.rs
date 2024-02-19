use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[allow(dead_code)]
impl User {
    pub fn new(id: Uuid, username: String, email: String, password: String) -> User {
        User {
            id,
            username,
            email,
            password,
        }
    }

    pub fn from_row(row: PgRow) -> User {
        let id = row.get::<Uuid, &str>("id");
        let username = row.get::<String, &str>("username");
        let email = row.get::<String, &str>("email");
        let password = row.get::<String, &str>("password");

        User {
            id,
            username,
            email,
            password,
        }
    }
}
