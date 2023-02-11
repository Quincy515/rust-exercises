use axum::http::StatusCode;
use bcrypt::hash;

use crate::util::app_error::AppError;
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password.as_bytes(), bcrypt::DEFAULT_COST).map_err(|err| {
        eprintln!("Error hashing password: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error securing password")
    })
}
