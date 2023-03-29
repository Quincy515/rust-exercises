use axum::{extract::State, Json};

use entity::{tasks, users};
use sea_orm::{DatabaseConnection, Set};
use types::user::{RequestCreateUser, ResponseDataUser, ResponseUser};

use crate::queries::task_queries::{get_default_task, save_active_task};
use crate::queries::user_queries::save_active_user;
use crate::util::{app_error::AppError, hash::hash_password, jwt::create_token};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(secret): State<String>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.username = Set(request_user.username.clone());
    new_user.password = Set(hash_password(&request_user.password)?);
    new_user.token = Set(Some(create_token(&secret, request_user.username)?));
    let user = save_active_user(&db, new_user).await?;

    create_default_tasks_for_user(&db, &user).await?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap_or_default(),
        },
    }))
}

async fn create_default_tasks_for_user(
    db: &DatabaseConnection,
    user: &users::Model,
) -> Result<(), AppError> {
    let default_tasks = get_default_task(db).await?;

    Ok(for default_task in default_tasks {
        let task = tasks::ActiveModel {
            priority: Set(default_task.priority),
            title: Set(default_task.title),
            completed_at: Set(default_task.completed_at),
            description: Set(default_task.description),
            deleted_at: Set(default_task.deleted_at),
            user_id: Set(Some(user.id)),
            ..Default::default()
        };

        save_active_task(&db, task).await?;
    })
}
