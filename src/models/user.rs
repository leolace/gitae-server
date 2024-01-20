use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(id: i32, username: String, email: String, password: String) -> User {
        User {
            id,
            username,
            email,
            password,
        }
    }

    pub fn from_row(row: PgRow) -> User {
        let id = row.get::<i32, &str>("id");
        let username = row.get::<String, &str>("username");
        let email = row.get::<String, &str>("email");
        let password = row.get::<String, &str>("password");

        User {
            username,
            id,
            email,
            password,
        }
    }
}
