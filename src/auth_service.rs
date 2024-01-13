use crate::auth_controller::{SignUp, User};
use crate::DbPool;
use actix_web::web;
use postgres::Row;
use std::error::Error;

pub async fn create(body: web::Json<SignUp>, pool: web::Data<DbPool>) -> Result<User, String> {
    let results = web::block(move || {
        let mut client = match pool.get() {
            Ok(c) => c,
            Err(e) => panic!("Error when getting client: {}", e),
        };
         match client.query_one(
            "INSERT INTO users (username, email, password) 
                VALUES($1, $2, $3) 
                RETURNING id, username, email, password",
            &[&body.username, &body.email, &body.password],
        ) {
            Ok(v) => v,
            Err(e) => panic!("Error when running query: {}", e) 
        }

    })
    .await;


    match results {
        Ok(v) => Ok(User::from_row(v)),
        Err(e) => panic!("Something got wrong: {}", e),
    }
}
