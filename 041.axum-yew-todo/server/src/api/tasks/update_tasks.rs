use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use chrono::Utc;
use entity::tasks::{self, Entity as Tasks};
use entity::users::Model as UserModel;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::util::app_error::AppError;

pub async fn mark_completed(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting task to update: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happend")
        })?;
    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Task not found"));
    };

    let now = Utc::now();
    task.completed_at = Set(Some(now.into()));
    task.save(&db).await.map_err(|err| {
        eprintln!("Error marking task as completed: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while updating completed at",
        )
    })?;
    Ok(())
}
