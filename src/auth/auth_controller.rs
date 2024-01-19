use crate::auth::{auth_dto, auth_service::AuthService};
use crate::error::Error;
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::postgres::PgPool;

pub async fn sign_in(pool: web::Data<PgPool>) -> HttpResponse {
    // TODO: user login
    HttpResponse::Ok().finish()
}

pub async fn sign_up(body: web::Json<auth_dto::SignUp>, pool: web::Data<PgPool>) -> HttpResponse {
    let c = AuthService::new(pool).create(body).await;

    match c {
        Ok(c) => HttpResponse::Created().json(c),
        Err(e) => HttpResponse::build(e.code).json(e),
    }
}
