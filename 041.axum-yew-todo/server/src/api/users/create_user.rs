use axum::{extract::State, http::StatusCode, Json};
use entity::tasks::Entity as Tasks;
use entity::{tasks, users};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TryIntoModel,
};
use types::user::{RequestCreateUser, ResponseDataUser, ResponseUser};

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
    let user = new_user
        .save(&db)
        .await
        .map_err(|err| {
            let error_message = err.to_string();
            if error_message
                .contains("duplicate key value violates unique constraint \"users_username_key\"")
            {
                AppError::new(
                    StatusCode::BAD_REQUEST,
                    "Username already taken, try again with a different user name",
                )
            } else {
                eprintln!("Error creating user: {:?}", &err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
        })?
        .try_into_model()
        .map_err(|err| {
            eprintln!("Error converting user back into model: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        })?;

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
    let default_tasks = Tasks::find()
        .filter(tasks::Column::IsDefault.eq(Some(true)))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(db)
        .await
        .map_err(|err| {
            eprintln!("Error getting default tasks: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error applying default tasks to new account",
            )
        })?;
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

        task.save(db).await.map_err(|err| {
            eprintln!("Error creating task from default: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error saving new default task for user",
            )
        })?;
    })
}
