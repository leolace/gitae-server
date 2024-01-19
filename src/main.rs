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

pub type AppPool = web::Data<PgPool>;

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

async fn get_pool() -> PgPool {
    println!("chegou");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    pool
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
            .service(index)
    })
    .on_connect(|_, _| println!("conexão estabelecida"))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
