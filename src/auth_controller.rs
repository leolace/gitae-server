use crate::auth_service;
use crate::DbPool;
use actix_web::{web, HttpRequest, HttpResponse};
use postgres::Row;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct SignUp {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new() {}

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

pub async fn find(pool: web::Data<DbPool>) -> HttpResponse {
    // TODO: user login
    HttpResponse::Ok().finish()
}

pub async fn create(body: web::Json<SignUp>, pool: web::Data<DbPool>) -> HttpResponse {
    let user = auth_service::create(body, pool).await.unwrap();
    HttpResponse::Ok().json(user)
}
