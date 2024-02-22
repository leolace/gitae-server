use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::error::ErrorMessage;

use super::{curriculum_dto::Store, curriculum_service::CurriculumService};

pub async fn store(body: Option<web::Json<Store>>, pool: web::Data<PgPool>) -> HttpResponse {
    let body = match body {
        Some(body) => body,
        None => return HttpResponse::BadRequest().json(ErrorMessage::new("Invalid request")),
    };

    let curriculum = CurriculumService::new(pool).store(body).await;

    match curriculum {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => HttpResponse::build(e.code).json(e),
    }
}
