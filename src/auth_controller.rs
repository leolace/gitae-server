use crate::auth_service::AuthService;
use actix_web::{web, HttpRequest, HttpResponse};
use postgres::Row;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::postgres::PgPool;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignUp {
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

    pub fn from_row(row: Row) -> User {
        let id = row.get::<&str, i32>("id");
        let username = row.get::<&str, String>("username");
        let email = row.get::<&str, String>("email");
        let password = row.get::<&str, String>("password");

        User {
            username,
            id,
            email,
            password,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Error {
    code: u32,
    message: String,
}

impl Error {
    pub fn new(code: u32, message: String) -> Error {
        Error { code, message }
    }
}

pub async fn find(pool: web::Data<PgPool>) -> HttpResponse {
    // TODO: user login
    HttpResponse::Ok().finish()
}

pub async fn create(body: web::Json<SignUp>, pool: web::Data<PgPool>) -> HttpResponse {
    let c = AuthService::create(body, pool).await;

    HttpResponse::Ok().json(c)
}
