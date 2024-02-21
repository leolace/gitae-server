use crate::error::HttpError;
use crate::models::user::User;
use crate::{AppPool, ResultE};
use actix_web::http::StatusCode;
use sqlx::{self, Row};
use uuid::Uuid;

pub struct UserService {
    pool: AppPool,
}

impl UserService {
    pub async fn new(pool: AppPool) -> UserService {
        UserService { pool }
    }

    pub async fn index(&self) -> ResultE<Vec<User>> {
        let pool = self.pool.get_ref();

        let users_query = sqlx::query("SELECT * FROM users").fetch_all(pool).await;
        let users_from_db = match users_query {
            Ok(users_from_db) => users_from_db,
            Err(_) => return Err(HttpError::new(StatusCode::NOT_FOUND, "No users found")),
        };

        let mut users: Vec<User> = Vec::new();

        for user in users_from_db {
            let user_from_row = User::from_row(user);
            users.push(user_from_row);
        }

        Ok(users)
    }

    pub async fn find(&self, id: Uuid) -> Option<User> {
        let pool = self.pool.get_ref();

        match sqlx::query("SELECT * FROM users WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
        {
            Ok(user) => Some(User::from_row(user)),
            Err(_) => None,
        }
    }

    pub async fn delete(&self, id: Uuid) -> ResultE<()> {
        let pool = self.pool.get_ref();

        let delete = sqlx::query("DELETE FROM users WHERE id = ($1)")
            .bind(id)
            .execute(pool)
            .await;

        match delete {
            Ok(_) => Ok(()),
            Err(_) => Err(HttpError::new(
                StatusCode::NOT_FOUND,
                "No users found or was not possible to delete",
            )),
        }
    }

    pub async fn find_by_email(&self, email: &String) -> Option<User> {
        let pool = self.pool.get_ref();

        match sqlx::query("SELECT * FROM users WHERE email=$1")
            .bind(email)
            .fetch_one(pool)
            .await
        {
            Ok(user) => Some(User::from_row(user)),
            Err(_) => None,
        }
    }

    pub async fn exists_by_email(&self, email: &String) -> bool {
        let pool = self.pool.get_ref();

        let query = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email=$1)")
            .bind(email)
            .fetch_one(pool)
            .await
            .unwrap();

        query.get::<bool, &str>("exists")
    }

    pub async fn exists_by_username(&self, username: &String) -> bool {
        let pool = self.pool.get_ref();

        let query = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE username=$1)")
            .bind(username)
            .fetch_one(pool)
            .await
            .unwrap();

        query.get::<bool, &str>("exists")
    }
}
