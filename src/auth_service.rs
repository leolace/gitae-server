use crate::auth_controller::{SignUp, User};
use crate::DbPool;
use actix_web::web;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use r2d2::PooledConnection;

async fn get_client(pool: web::Data<DbPool>) -> PooledConnection<PostgresConnectionManager<NoTls>> {
    let client = web::block(move || {
        let client = pool.get().unwrap();
        client
    })
    .await
    .unwrap();

    client
}

pub async fn create(body: web::Json<SignUp>, pool: web::Data<DbPool>) -> Result<User, String> {
    let mut client = get_client(pool).await; 

    let query = web::block(move || {
        match client.query_one(
            "INSERT INTO users (username, email, password) 
                VALUES($1, $2, $3) 
                RETURNING id, username, email",
            &[&body.username, &body.email, &body.password],
        ) {
            Ok(v) => v,
            Err(e) => panic!("Error when running query: {}", e),
        }
    })
    .await;

    match query {
        Ok(query) => Ok(User::from_row(query)),
        Err(e) => Err(String::from("erro")),
    }
}
