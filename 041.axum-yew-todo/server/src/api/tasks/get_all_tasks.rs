use axum::http::StatusCode;
use axum::{extract::State, Extension, Json};
use entity::users::Model as UserModel;
use entity::{prelude::*, tasks};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use types::task::{ResponseDataTasks, ResponseTask};

use crate::util::app_error::AppError;

pub async fn get_all_tasks(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataTasks>, AppError> {
    let tasks = Tasks::find()
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .filter(tasks::Column::DeletedAt.is_null())
        // .into_model::<ResponseTask>()
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting all tasks: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error getting all tasks")
        })?
        .into_iter()
        .map(|task| ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        })
        .collect::<Vec<ResponseTask>>();

    Ok(Json(ResponseDataTasks { data: tasks }))
}
