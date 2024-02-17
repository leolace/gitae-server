use actix_cors::Cors;
use actix_web::{get, http, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;

mod routes;

mod user;
mod auth;
mod models;

mod error;

pub type ResultE<T, E = error::HttpError> = Result<T, E>;

pub type AppPool = web::Data<PgPool>;

async fn get_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgPool::connect(&database_url).await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = get_pool().await;

    println!("🔥 Server is running!");
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::user_routes)
            .configure(routes::auth_routes)
    })
    .on_connect(|_, _| println!("conexão estabelecida"))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
