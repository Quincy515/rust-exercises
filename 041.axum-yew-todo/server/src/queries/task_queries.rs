use axum::http::StatusCode;
use entity::{prelude::*, tasks, tasks::Model as TasksModel, users::Model as UsersModel};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TryIntoModel,
};

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

    save_active_task(&db, new_task).await
}

pub async fn find_task_by_id(
    db: &DatabaseConnection,
    id: i32,
    user_id: i32,
) -> Result<TasksModel, AppError> {
    let task = Tasks::find_by_id(id)
        .filter(tasks::Column::UserId.eq(Some(user_id)))
        .one(db)
        .await
        .map_err(|err| {
            eprintln!("Error getting task by id: {err:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting your task",
            )
        })?;

    task.ok_or_else(|| {
        eprintln!("CCould not find task by id: {id:?}");
        AppError::new(StatusCode::NOT_FOUND, "Task not found")
    })
}

pub async fn save_active_task(
    db: &DatabaseConnection,
    task: tasks::ActiveModel,
) -> Result<TasksModel, AppError> {
    let task = task.save(db).await.map_err(|err| {
        eprintln!("Error saving task: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving task")
    })?;
    convert_active_to_model(task)
}

pub async fn get_all_tasks(
    db: &DatabaseConnection,
    user_id: i32,
    delete: bool,
) -> Result<Vec<TasksModel>, AppError> {
    let mut query = Tasks::find().filter(tasks::Column::UserId.eq(Some(user_id)));
    if delete {
        query = query.filter(tasks::Column::DeletedAt.is_null());
    }

    query
        // .into_model::<ResponseTask>()
        .all(db)
        .await
        .map_err(|err| {
            eprintln!("Error getting all tasks: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error getting all tasks")
        })
}

fn convert_active_to_model(active_task: tasks::ActiveModel) -> Result<TasksModel, AppError> {
    active_task.try_into_model().map_err(|err| {
        eprintln!("Error converting task active model to model: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    })
}
