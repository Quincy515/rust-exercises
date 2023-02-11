use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    exp: usize,
    username: String,
}

pub fn create_token(secret: &str, username: String) -> Result<String, AppError> {
    let now = chrono::Utc::now();
    let expires_at = now + Duration::hours(1);
    let exp = expires_at.timestamp() as usize;
    let claims = Claims { exp, username };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&token_header, &claims, &key).map_err(|err| {
        eprintln!("Error creating token: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error creating the token",
        )
    })
}
