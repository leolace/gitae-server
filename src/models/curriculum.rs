use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::postgres::PgRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Curriculum {
    pub id: Uuid,
    pub user_id: Uuid,
    pub github_user_id: String,
    pub name: String,
    pub job_title: String,
    pub about: String,
    pub skills: Vec<String>,
}

impl Curriculum {
    pub fn from_row(row: PgRow) -> Curriculum {
        let id = row.get::<Uuid, &str>("id");
        let user_id = row.get::<Uuid, &str>("user_id");
        let github_user_id = row.get::<String, &str>("github_user_id");
        let name = row.get::<String, &str>("name");
        let job_title = row.get::<String, &str>("job_title");
        let about = row.get::<String, &str>("about");
        let skills = row.get::<Vec<String>, &str>("skills");

        Curriculum {
            id,
            user_id,
            github_user_id,
            name, 
            job_title,
            about,
            skills
        }
    }
}
