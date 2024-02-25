use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Store {
    pub user_id: Uuid,
    pub github_user_id: String,
    pub name: String,
    pub job_title: String,
    pub about: String,
    pub skills: Vec<String>,
}
