use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::postgres::PgRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Curriculum {
    pub id: Uuid,
    pub user_id: Uuid,
    pub github_curriculum_id: String,
    pub name: String,
    pub job_title: String,
    pub about: String,
    pub skills: Vec<String>,
}

impl Curriculum {
    pub fn from_row(row: PgRow) -> Curriculum {
        let id = row.get::<Uuid, &str>("id");
        let user_id = row.get::<Uuid, &str>("user_id");
        let github_curriculum_id = row.get::<i32, &str>("github_curriculum_id");
        let name = row.get::<String, &str>("name");
        let job_title = row.get::<String, &str>("job_title");
        let about = row.get::<String, &str>("about");
        let skills = row.get::<Vec<String>, &str>("skills");

        Curriculum {
            id,
            user_id,
            github_curriculum_id: github_curriculum_id.to_string(),
            name, 
            job_title,
            about,
            skills
        }
    }
}
