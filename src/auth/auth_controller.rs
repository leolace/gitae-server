use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;
use crate::auth::auth_service::AuthService;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignUp {
    #[serde(default)]
    pub username: String,

    #[serde(default)]
    pub email: String,

    #[serde(default)]
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

#[derive(Deserialize, Serialize)]
pub struct Error {
    code: u32,
    message: String,
}

impl Error {
    pub fn new(code: u32, message: String) -> Error {
        Error { code, message }
    }
}

pub async fn sign_in(pool: web::Data<PgPool>) -> HttpResponse {
    // TODO: user login
    HttpResponse::Ok().finish()
}

pub async fn sign_up(body: web::Json<SignUp>, pool: web::Data<PgPool>) -> HttpResponse {
    let c = AuthService::new(pool).create(body).await;

    match c {
        Ok(c) => HttpResponse::Created().json(c),
        Err(e) => e,
    }
}
