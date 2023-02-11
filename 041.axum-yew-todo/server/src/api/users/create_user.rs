use axum::{extract::State, http::StatusCode, Json};
use entity::users;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};
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

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap_or_default(),
        },
    }))
}
