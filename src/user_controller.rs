use actix_web::{get, web, HttpResponse, Responder, Result};
use r2d2;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use serde::Deserialize;
pub type DbPool = r2d2::Pool<PostgresConnectionManager<NoTls>>;

#[derive(Deserialize)]
pub struct User {
    username: String,
    age: usize,
}

pub async fn get_user(info: web::Path<String>, pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let user_id = info.into_inner();
    let query = web::block(move || {
        let mut client = pool.get().unwrap();
        client.query("SELECT * FROM users", &[]).unwrap()
    })
    .await?;

    for row in query {
        println!("username: {:#?}", row.get::<&str, String>("username"));
        println!("email: {:#?}", row.get::<&str, String>("email"));
        println!("password: {:#?}", row.get::<&str, String>("password"));
    }

    Ok(HttpResponse::Ok().body(""))
}

pub async fn add_user(data: web::Json<User>) -> Result<String> {
    let username = &data.username;
    let age = data.age + 10;
    Ok(format!("usuário: {}, idade: {}", username, age))
}
