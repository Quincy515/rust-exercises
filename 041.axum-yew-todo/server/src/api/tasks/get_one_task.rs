use axum::{
    extract::{Path, State},
    Extension, Json,
};
use entity::users::Model as UserModel;

use sea_orm::DatabaseConnection;
use types::task::{ResponseDataTask, ResponseTask};

use crate::{queries::task_queries::find_task_by_id, util::app_error::AppError};

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<Json<ResponseDataTask>, AppError> {
    let task = find_task_by_id(&db, task_id, user.id).await?;

    let response = ResponseTask {
        id: task.id,
        title: task.title,
        priority: task.priority,
        description: task.description,
        completed_at: task.completed_at.map(|time| time.to_string()),
    };

    Ok(Json(ResponseDataTask { data: response }))
}
