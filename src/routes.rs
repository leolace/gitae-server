use crate::auth::auth_controller;
use crate::curriculum::curriculum_controller;
use crate::user::user_controller;
use actix_web::web;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/", web::get().to(user_controller::index))
            .route("/{id}", web::get().to(user_controller::find))
            .route("/{id}", web::delete().to(user_controller::delete))
            .route(
                "/{user_id}/curriculums",
                web::get().to(curriculum_controller::find_all_by_user),
            ),
    );
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signin", web::post().to(auth_controller::sign_in))
            .route("/signup", web::post().to(auth_controller::sign_up))
            .route("/me", web::get().to(auth_controller::me)),
    );
}

pub fn curriculum_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/curriculum")
            .route("/", web::post().to(curriculum_controller::store))
            .route(
                "/{curriculum_id}",
                web::get().to(curriculum_controller::find_one),
            ),
    );
}
