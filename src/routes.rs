use actix_web::web;
use crate::auth::auth_controller;

pub fn get_hello() {
    println!("hello world");
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(
    //     web::scope("/users")
    //         .route("/{userid}", web::get().to(user_controller::get_user))
    //         .route("", web::post().to(user_controller::add_user)),
    // );
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/auth")
            .route("/signin", web::post().to(auth_controller::sign_in))
            .route("/signup", web::post().to(auth_controller::sign_up))
            .route("/me", web::get().to(auth_controller::me)),
    );
}
