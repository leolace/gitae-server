use crate::AppPool;
use actix_web::{get, web, HttpResponse, Responder, Result};
use crate::models::user::User;

pub async fn get_user(info: web::Path<String>, pool: web::Data<AppPool>) -> Result<HttpResponse> {
    let user_id = info.into_inner();

    Ok(HttpResponse::Ok().finish())
}

pub async fn add_user(data: web::Json<User>)  {
}
