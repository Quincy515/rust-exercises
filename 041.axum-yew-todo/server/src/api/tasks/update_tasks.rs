use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::Utc;
use entity::tasks::{self, Entity as Tasks};
use entity::users::Model as UserModel;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use types::task::RequestTask;

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

pub async fn mark_uncompleted(
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

    task.completed_at = Set(None);
    task.save(&db).await.map_err(|err| {
        eprintln!("Error marking task as uncompleted: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while updating uncompleted at",
        )
    })?;
    Ok(())
}

pub async fn update_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
    Json(request_task): Json<RequestTask>,
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

    if let Some(priority) = request_task.priority {
        task.priority = Set(priority);
    }
    if let Some(description) = request_task.description {
        task.description = Set(description);
    }
    if let Some(title) = request_task.title {
        task.title = Set(title);
    }
    if let Some(completed_at) = request_task.completed_at {
        task.completed_at = Set(completed_at);
    }

    task.save(&db).await.map_err(|err| {
        eprintln!("Error marking task as uncompleted: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while updating uncompleted at",
        )
    })?;
    Ok(())
}
