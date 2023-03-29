use axum::{extract::State, http::StatusCode, Extension};
use entity::users;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

use crate::{queries::user_queries::save_active_user, util::app_error::AppError};

pub async fn logout(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
) -> Result<StatusCode, AppError> {
    let mut user = user.into_active_model();
    user.token = Set(None);
    save_active_user(&db, user).await?;
    Ok(StatusCode::OK)
}
