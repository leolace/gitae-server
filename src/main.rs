use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use r2d2;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::env;
use std::process;
use postgres::config::Config;

pub type DbPool = r2d2::Pool<PostgresConnectionManager<NoTls>>;

mod routes;
mod user_controller;

mod auth_controller;
mod auth_service;

#[get("/get")]
async fn get_funtion() -> HttpResponse {
    HttpResponse::Ok().body("hello world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    println!("{:?}", req.query_string());
    HttpResponse::Ok().body("index mudou")
}

async fn get_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let config = Config::from(database_url.parse::<Config>().unwrap());

    let manager = PostgresConnectionManager::new(config, NoTls);

    let pool = r2d2::Pool::new(manager).unwrap_or_else(|e| {
        println!("Something got wrong: {}", e);
        process::exit(1)
    });

    pool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let client = get_pool().await;

    routes::get_hello();
    println!("🔥 Server is running!");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(routes::user_routes)
            .configure(routes::auth_routes)
            .service(web::scope("/nested").service(get_funtion).service(echo))
            .service(index)
    })
    .on_connect(|_, _| println!("conexão estabelecida"))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
