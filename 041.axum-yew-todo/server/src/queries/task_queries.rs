use axum::http::StatusCode;
use entity::{tasks, tasks::Model as TasksModel, users::Model as UsersModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};

use crate::{api::tasks::create_task_extractor::ValidateCreateTask, util::app_error::AppError};

pub async fn create_task(
    task: ValidateCreateTask,
    user: &UsersModel,
    db: &DatabaseConnection,
) -> Result<TasksModel, AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title.unwrap_or_default()),
        description: Set(task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let task = new_task.save(db).await.map_err(|err| {
        eprintln!("Error creating task: {:?}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating task")
    })?;

    convert_active_to_model(task)
}

fn convert_active_to_model(active_task: tasks::ActiveModel) -> Result<TasksModel, AppError> {
    active_task.try_into_model().map_err(|err| {
        eprintln!("Error converting task active model to model: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    })
}
