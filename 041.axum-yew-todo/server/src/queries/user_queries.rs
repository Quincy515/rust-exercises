use axum::http::StatusCode;
use entity::{
    prelude::*,
    users::{self, Model as UsersModel},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TryIntoModel,
};

use crate::util::app_error::AppError;

pub async fn save_active_user(
    db: &DatabaseConnection,
    user: users::ActiveModel,
) -> Result<UsersModel, AppError> {
    let user = user.save(db).await.map_err(|err| {
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
    })?;

    convert_active_to_model(user)
}

pub async fn find_by_username(
    db: &DatabaseConnection,
    username: String,
) -> Result<UsersModel, AppError> {
    Users::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|err| {
            eprintln!("Error getting user by username: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error loggin in")
        })?
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "bad username or password"))
}

fn convert_active_to_model(active_user: users::ActiveModel) -> Result<UsersModel, AppError> {
    active_user.try_into_model().map_err(|err| {
        eprintln!("Error converting user active model to model: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    })
}
