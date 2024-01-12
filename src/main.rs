use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

mod routes;
mod user_controller;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    routes::get_hello();
    println!("🔥 Server is running!");
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(String::from("hello world")))
            .configure(routes::user_routes)
            .service(web::scope("/nested").service(get_funtion).service(echo))
            .service(index)
    })
    .on_connect(|_, _| println!("conexão estabelecida"))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
