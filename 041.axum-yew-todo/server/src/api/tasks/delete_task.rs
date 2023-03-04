use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use entity::{prelude::*, tasks, users::Model as UserModel};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::util::app_error::AppError;

pub async fn soft_delete_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error deleteing task: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error deleting the task")
        })?;

    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Task not found"));
    };

    let now = chrono::Utc::now();

    task.deleted_at = Set(Some(now.into()));

    task.save(&db).await.map_err(|err| {
        eprintln!("Error deleting task: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while deleting task",
        )
    })?;

    Ok(())
}
