use crate::user_controller;
use actix_web::web;

pub fn get_hello() {
    println!("hello world");
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/{userid}")
        .route(web::get().to(user_controller::get_user))
    );

    cfg.service(web::resource("/user")
        .route(web::post().to(user_controller::add_user))
    );
}
