use axum::{extract::State, http::StatusCode, Json};
use entity::{prelude::*, users};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set, TryIntoModel,
};
use types::user::{RequestCreateUser, ResponseDataUser, ResponseUser};

use crate::util::{app_error::AppError, hash::verify_password, jwt::create_token};

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(secret): State<String>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let user = Users::find()
        .filter(users::Column::Username.eq(request_user.username.as_str()))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting user for logging in: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error loggin in")
        })?;

    if let Some(user) = user {
        if !verify_password(&request_user.password, &user.password)? {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "bad username or password",
            ));
        }
        let token = create_token(&secret, user.username.clone())?;
        let mut user = user.into_active_model();
        user.token = Set(Some(token));
        let user = user
            .save(&db)
            .await
            .map_err(|err| {
                eprintln!("Error saving user token: {:?}", err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving user token")
            })?
            .try_into_model()
            .map_err(|err| {
                eprintln!("Error converting model to active model: {:?}", err);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error converting model to active model",
                )
            })?;
        let response = ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap_or_default(),
        };

        Ok(Json(ResponseDataUser { data: response }))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "User not found"))
    }
}
