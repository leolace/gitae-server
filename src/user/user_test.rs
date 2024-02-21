#[cfg(test)]
mod user_tests {
    use super::*;
    use crate::{helpers::init_test, models::user::User};
    use actix_web::test::{self, TestRequest};

    #[actix_web::test]
    async fn index() {
        let mut app = init_test::test::init().await;

        let resp = TestRequest::get()
            .uri("/user/")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success())
    }

    #[actix_web::test]
    async fn find() {
        let mut app = init_test::test::init().await;

        let resp = TestRequest::get()
            .uri("/user/")
            .send_request(&mut app)
            .await;

        let users: Vec<User> = test::read_body_json(resp).await;
        let first_user = users.first().unwrap();

        let resp = TestRequest::get()
            .uri(&("/user/".to_owned() + &first_user.id.to_string()))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success())
    }
}
