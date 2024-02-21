use crate::user::user_service::UserService;
use crate::PgPool;
use actix_web::{web, HttpResponse};
use uuid::Uuid;

pub async fn index(pool: web::Data<PgPool>) -> HttpResponse {
    let users = UserService::new(pool).await.index().await;

    match users {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => HttpResponse::build(e.code).json(e),
    }
}

pub async fn delete(pool: web::Data<PgPool>, path: web::Path<(Uuid)>) -> HttpResponse {
    let (id) = path.into_inner();
    let users = UserService::new(pool).await.delete(id).await;

    match users {
        Ok(d) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::build(e.code).json(e),
    }
}
