use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;

mod auth;
mod error;
mod models;
mod routes;
mod user;

pub type ResultE<T, E = error::HttpError> = Result<T, E>;
pub type AppPool = web::Data<PgPool>;

async fn get_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    env::var("SECRET_JWT").expect("SECRET_JWT not set");

    match PgPool::connect(&database_url).await {
        Ok(pool) => pool,
        Err(e) => panic!("{}", e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    let pool = get_pool().await;
    env_logger::init();

    println!("🔥 Server is running!");
    HttpServer::new(move || {
        let cors = Cors::permissive();
        let logger = Logger::new("%a %r %s %Tsec");

        App::new()
            .wrap(cors)
            .wrap(logger)
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::user_routes)
            .configure(routes::auth_routes)
    })
    .on_connect(|_, _| println!("conexão estabelecida"))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
