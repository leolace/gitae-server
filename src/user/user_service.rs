use crate::models::user::User;
use crate::AppPool;
use sqlx;
use sqlx::Row;

pub struct UserService {
    pool: AppPool,
}

impl UserService {
    pub async fn new(pool: AppPool) -> UserService {
        UserService { pool }
    }

    pub async fn find(&self, id: i32) -> Option<User> {
        let pool = self.pool.get_ref();

        match sqlx::query("SELECT * FROM users WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
        {
            Ok(d) => Some(User::from_row(d)),
            Err(_) => None,
        }
    }

    pub async fn find_by_email(&self, email: &String) -> Option<User> {
        let pool = self.pool.get_ref();

        match sqlx::query("SELECT * FROM users WHERE email=$1")
            .bind(email)
            .fetch_one(pool)
            .await
        {
            Ok(d) => Some(User::from_row(d)),
            Err(_) => None,
        }
    }
}
