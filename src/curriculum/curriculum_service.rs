use actix_http::StatusCode;
use actix_web::web;
use sqlx;
use uuid::Uuid;

use crate::{
    error::HttpError, models::curriculum::Curriculum, user::user_service::UserService, AppPool,
    ResultE,
};

use super::curriculum_dto::Store;

pub struct CurriculumService {
    pool: AppPool,
}

impl CurriculumService {
    pub fn new(pool: AppPool) -> CurriculumService {
        CurriculumService { pool }
    }

    pub async fn store(&self, body: web::Json<Store>) -> ResultE<Curriculum> {
        let pool = self.pool.get_ref();
        let user_dto: Store = body.clone();

        let user_exists = UserService::new(self.pool.clone())
            .await
            .find(user_dto.user_id)
            .await;

        match user_exists {
            Some(_) => (),
            None => return Err(HttpError::new(StatusCode::NOT_FOUND, "User not found")),
        }

        let query = sqlx::query(
                "
                INSERT INTO curriculums (user_id, github_curriculum_id, name, job_title, about, skills)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING *
                ",
            )
            .bind(user_dto.user_id)
            .bind(user_dto.github_curriculum_id.parse::<i32>().unwrap())
            .bind(user_dto.name)
            .bind(user_dto.job_title)
            .bind(user_dto.about)
            .bind(user_dto.skills)
            .fetch_one(pool)
            .await;

        match query {
            Ok(curriculum) => Ok(Curriculum::from_row(curriculum)),
            Err(_) => Err(HttpError::new(
                StatusCode::BAD_REQUEST,
                "Was not possible to create curriculum",
            )),
        }
    }

    pub async fn find_one(&self, curriculum_id: Uuid) -> ResultE<Curriculum> {
        let pool = self.pool.get_ref();

        let query = sqlx::query("SELECT * FROM curriculums WHERE id = ($1)")
            .bind(curriculum_id)
            .fetch_one(pool)
            .await;

        match query {
            Ok(curriculum) => Ok(Curriculum::from_row(curriculum)),
            Err(_) => Err(HttpError::new(
                StatusCode::NOT_FOUND,
                "Curriculum not found",
            )),
        }
    }

    pub async fn find_all_by_user(&self, user_id: Uuid) -> ResultE<Vec<Curriculum>> {
        let pool = self.pool.get_ref();

        let user_exists = UserService::new(self.pool.clone())
            .await
            .find(user_id)
            .await;

        match user_exists {
            Some(_) => (),
            None => return Err(HttpError::new(StatusCode::NOT_FOUND, "User not found")),
        };

        let query = sqlx::query("SELECT * FROM curriculums WHERE user_id = ($1)")
            .bind(user_id)
            .fetch_all(pool)
            .await;

        match query {
            Ok(rows) => {
                let mut curriculums: Vec<Curriculum> = Vec::new(); 
                for row in rows {
                    curriculums.push(Curriculum::from_row(row))
                }
                Ok(curriculums)
            },
            Err(_) => Err(HttpError::new(StatusCode::BAD_REQUEST, "Something went wrong"))
        }
    }
}
