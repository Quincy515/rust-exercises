use axum::{extract::State, http::StatusCode, Extension};
use entity::users;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set};

use crate::util::app_error::AppError;

pub async fn logout(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
) -> Result<StatusCode, AppError> {
    let mut user = user.into_active_model();
    user.token = Set(None);
    user.save(&db).await.map_err(|err| {
        eprintln!("Error removing token: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    })?;
    Ok(StatusCode::OK)
}
