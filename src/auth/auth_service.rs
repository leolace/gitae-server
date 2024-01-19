use crate::auth::{auth_controller, auth_dto};
use crate::error::Error;
use crate::models::user::User;
use crate::AppPool;
use actix_web::{http::StatusCode, web, HttpResponse};
use sqlx::Row;

pub trait RootService {
    async fn create(&self, body: web::Json<auth_dto::SignUp>, pool: AppPool) -> User;
}

pub struct AuthService {
    pool: AppPool,
}

impl AuthService {
    pub fn new(pool: AppPool) -> AuthService {
        AuthService { pool }
    }

    pub async fn create(&self, body: web::Json<auth_dto::SignUp>) -> Result<User, Error> {
        if body.email.is_empty() || body.username.is_empty() || body.password.is_empty() {
            return Err(Error::new(
                StatusCode::BAD_REQUEST,
                "All fields must be set",
            ));
        }

        let user_exists_by_email = self.exists_by_email(&body.email).await;

        if user_exists_by_email {
            return Err(Error::new(
                StatusCode::CONFLICT,
                "This email has already been taken",
            ));
        }

        let user_exists_by_username = self.exists_by_username(&body.username).await;

        if user_exists_by_username {
            return Err(Error::new(
                StatusCode::CONFLICT,
                "This username has already been taken",
            ));
        }

        let pool = self.pool.get_ref();
        let query = sqlx::query(
            "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&body.username)
        .bind(&body.email)
        .bind(&body.password)
        .fetch_one(pool)
        .await
        .unwrap();

        let user = User::from_row(query);

        Ok(user)
    }

    async fn exists_by_email(&self, email: &String) -> bool {
        let pool = self.pool.get_ref();

        let query = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email=$1)")
            .bind(email)
            .fetch_one(pool)
            .await
            .unwrap();

        let exists = query.get::<bool, &str>("exists");

        exists
    }

    async fn exists_by_username(&self, username: &String) -> bool {
        let pool = self.pool.get_ref();

        let query = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE username=$1)")
            .bind(username)
            .fetch_one(pool)
            .await
            .unwrap();

        let exists = query.get::<bool, &str>("exists");

        exists
    }
}