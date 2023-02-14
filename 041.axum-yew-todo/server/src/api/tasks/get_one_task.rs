use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use entity::users::Model as UserModel;
use entity::{prelude::*, tasks};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use types::task::{ResponseDataTask, ResponseTask};

use crate::util::app_error::AppError;

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<Json<ResponseDataTask>, AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        // .into_model::<ResponseTask>()
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error when getting task: {err:?}");
            AppError::new(
                StatusCode::BAD_REQUEST,
                "We got an error when attempting to load your task",
            )
        })?;

    let response = if let Some(task) = task {
        // task
        ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        }
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };

    Ok(Json(ResponseDataTask { data: response }))
}
