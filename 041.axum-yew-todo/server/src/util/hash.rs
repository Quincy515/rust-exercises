use axum::http::StatusCode;
use bcrypt::{hash, verify};

use crate::util::app_error::AppError;
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password.as_bytes(), bcrypt::DEFAULT_COST).map_err(|err| {
        eprintln!("Error hashing password: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error securing password")
    })
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(|err| {
        eprintln!("Error verifying password: {err:?}");
        AppError::new(
            StatusCode::BAD_REQUEST,
            "There was an error verifying the password",
        )
    })
}
