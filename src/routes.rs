use crate::user_controller;
use actix_web::web;

pub fn get_hello() {
    println!("hello world");
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/{userid}", web::get().to(user_controller::get_user))
            .route("", web::post().to(user_controller::add_user)),
    );
}
