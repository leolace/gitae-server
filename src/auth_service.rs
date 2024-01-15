use crate::auth_controller::{SignUp, User};
use crate::AppPool;
use actix_web::web;
use sqlx::{postgres::PgPool, Row};

pub trait RootService {
    async fn create(&self, body: web::Json<SignUp>, pool: AppPool) -> User;
}

pub struct AuthService {}

impl AuthService {
    pub async fn create(body: web::Json<SignUp>, pool: AppPool) -> User {
        let pool = pool.get_ref();
        let query = sqlx::query("SELECT * FROM users;").fetch_one(pool).await.unwrap();


        let user = query.get::<String, &str>("username");

        let user = User::new(
            10_i32,
            user,
            String::from("email"),
            String::from("senha"),
        );

        user
    }
}
