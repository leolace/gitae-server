#[cfg(test)]
mod auth_tests {
    use super::*;
    use crate::{
        auth::auth_dto::{SignIn, SignUp},
        helpers::init_test,
        models::{auth::AuthPayload, user::User},
    };
    use actix_web::{
        http::header::ContentType,
        test::{self, TestRequest},
    };

    #[actix_web::test]
    async fn sign_up() {
        let mut app = init_test::test::init().await;
        let user_req = SignUp {
            email: String::from("email@test.com"),
            username: String::from("test"),
            password: String::from("admin123"),
        };

        let req = TestRequest::post()
            .uri("/auth/signup")
            .insert_header(ContentType::json())
            .set_json(&user_req)
            .to_request();

        let resp = test::call_service(&app, req).await;

        let user: User = test::read_body_json(resp).await;

        TestRequest::delete()
            .uri(&("/user/".to_owned() + &user.id.to_string()))
            .send_request(&app)
            .await;

        assert_eq!(user.email, user_req.email);
    }

    #[actix_web::test]
    async fn sign_in() {
        let mut app = init_test::test::init().await;
        let signin = SignIn {
            email: String::from("leol@gmail.com"),
            password: String::from("admin123"),
        };

        let req = TestRequest::post()
            .uri("/auth/signin")
            .insert_header(ContentType::json())
            .set_json(signin)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn me() {
        let mut app = init_test::test::init().await;
        let signin = SignIn {
            email: String::from("teste7661@gmail.com"),
            password: String::from("teste3692"),
        };

        let req = TestRequest::post()
            .uri("/auth/signin")
            .insert_header(ContentType::json())
            .set_json(signin)
            .to_request();

        let resp = test::call_service(&app, req).await;

        let token: AuthPayload = test::read_body_json(resp).await;

        let req = TestRequest::get()
            .uri("/auth/me")
            .insert_header(("authorization", token.token))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }
}
