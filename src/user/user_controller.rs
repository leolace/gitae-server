use crate::AppPool;
use actix_web::{get, web, HttpResponse, Responder, Result};
use crate::models::user::User;
use crate::PgPool;
use crate::user::user_service::UserService;


pub async fn index(pool: web::Data<PgPool>) -> HttpResponse {
    let users = UserService::new(pool).await.index().await;

    match users {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => HttpResponse::build(e.code).json(e)
    }
}
