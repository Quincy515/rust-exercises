use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestTask {
    #[validate(
        required(message = "missing task title"),
        length(min = 1, max = 6, message = "task title length should >1 and <7")
    )]
    pub title: Option<String>,
    pub priority: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct ResponseTask {
    pub id: i32,
    pub title: String,
    pub priority: Option<String>,
    pub description: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseDataTask {
    pub data: ResponseTask,
}

#[derive(Serialize)]
pub struct ResponseDataTasks {
    pub data: Vec<ResponseTask>,
}
