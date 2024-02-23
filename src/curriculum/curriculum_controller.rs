use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

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

pub async fn find_one(
    curriculum_id: Option<web::Path<(Uuid)>>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let curriculum_id = match curriculum_id {
        Some(curriculum_id) => curriculum_id.to_owned(),
        None => return HttpResponse::BadRequest().json(ErrorMessage::new("Invalid request")),
    };

    let curriculum = CurriculumService::new(pool).find_one(curriculum_id).await;

    match curriculum {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => HttpResponse::build(e.code).json(e),
    }
}

pub async fn find_all_by_user(
    user_id: Option<web::Path<(Uuid)>>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let user_id = match user_id {
        Some(user_id) => user_id.to_owned(),
        None => return HttpResponse::BadRequest().json(ErrorMessage::new("Invalid request")),
    };

    let curriculum = CurriculumService::new(pool).find_all_by_user(user_id).await;

    match curriculum {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => HttpResponse::build(e.code).json(e),
    }
}
