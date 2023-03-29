use axum::{
    extract::{Path, State},
    Extension,
};
use entity::users::Model as UserModel;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

use crate::{
    queries::task_queries::{find_task_by_id, save_active_task},
    util::app_error::AppError,
};

pub async fn soft_delete_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<(), AppError> {
    let mut task = find_task_by_id(&db, task_id, user.id)
        .await?
        .into_active_model();

    let now = chrono::Utc::now();

    task.deleted_at = Set(Some(now.into()));

    save_active_task(&db, task).await?;

    Ok(())
}
