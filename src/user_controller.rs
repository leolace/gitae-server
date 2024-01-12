use actix_web::{get, web, HttpResponse, Responder, Result};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    username: String,
    age: usize,
}

pub async fn get_user(info: web::Path<String>, hello: web::Data<String>) -> Result<String> {
    let user_id = info.into_inner();
    Ok(format!("{}, {}", *hello, user_id))
}

pub async fn add_user(data: web::Json<User>) -> Result<String> {
    let username = &data.username;
    let age = data.age + 10;
    Ok(format!("usuário: {}, idade: {}", username, age))
}
