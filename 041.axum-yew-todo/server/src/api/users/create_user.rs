use axum::{extract::State, http::StatusCode, Json};
use entity::users;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use types::user::{RequestCreateUser, ResponseDataUser, ResponseUser};

use crate::util::app_error::AppError;

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.username = Set(request_user.username);
    new_user.password = Set(request_user.password);
    let user = new_user.save(&db).await.map_err(|error| {
        eprintln!("Error creating user: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    })?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id.unwrap(),
            username: user.username.unwrap(),
            token: user.token.unwrap().unwrap_or_default(),
        },
    }))
}
