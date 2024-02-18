use crate::auth::{auth_dto, auth_service::AuthService};
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::postgres::PgPool;

pub async fn sign_in(body: web::Json<auth_dto::SignIn>, pool: web::Data<PgPool>) -> HttpResponse {
    let token = AuthService::new(pool).sign_in(body).await;

    match token {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => HttpResponse::build(e.code).json(e),
    }
}

pub async fn sign_up(body: web::Json<auth_dto::SignUp>, pool: web::Data<PgPool>) -> HttpResponse {
    let user = AuthService::new(pool).sign_up(body).await;

    match user {
        Ok(d) => HttpResponse::Created().json(d),
        Err(e) => HttpResponse::build(e.code).json(e),
    }
}

pub async fn me(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let user = AuthService::new(pool).me(req).await;

    match user {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => HttpResponse::build(e.code).json(e),
    }
}
