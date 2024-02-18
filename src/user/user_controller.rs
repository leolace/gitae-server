use crate::user::user_service::UserService;
use crate::PgPool;
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<PgPool>) -> HttpResponse {
    let users = UserService::new(pool).await.index().await;

    match users {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => HttpResponse::build(e.code).json(e),
    }
}
