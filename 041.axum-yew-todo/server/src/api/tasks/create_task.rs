use axum::{extract::State, http::StatusCode, Extension, Json};
use entity::users::Model as UserModel;
use sea_orm::DatabaseConnection;
use types::task::{ResponseDataTask, ResponseTask};

use crate::{queries::task_queries, util::app_error::AppError};

use super::create_task_extractor::ValidateCreateTask;

pub async fn create_task(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
    task: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    let task = task_queries::create_task(task, &user, &db).await?;
    let response = ResponseDataTask {
        data: ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        },
    };
    Ok((StatusCode::CREATED, Json(response)))
}
