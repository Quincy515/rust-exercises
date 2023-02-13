use axum::{extract::State, http::StatusCode, Extension, Json};
use entity::{tasks, users::Model as UserModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};
use types::task::{ResponseDataTask, ResponseTask};

use crate::util::app_error::AppError;

use super::create_task_extractor::ValidateCreateTask;

pub async fn create_task(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
    // Json(request_task): Json<RequestTask>,
    task: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    /*
    if let Err(err) = request_task.validate() {
        let field_errors = err.field_errors();
        for (_, error) in field_errors {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                error
                    .first()
                    .unwrap()
                    .clone()
                    .message
                    .unwrap() // .unwrap_or_else(|| "Title shouldn't correct!".into())
                    .to_string(),
            ));
        }
    }
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title.unwrap_or_default()),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };
    */
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title.unwrap_or_default()),
        description: Set(task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };
    let task = new_task
        .save(&db)
        .await
        .map_err(|err| {
            eprintln!("Error creating task: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating task")
        })?
        .try_into_model()
        .map_err(|err| {
            eprintln!("Error converting task after creating it: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error converting task")
        })?;
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
