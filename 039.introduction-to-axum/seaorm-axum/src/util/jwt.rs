use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30);
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claim = Claims { exp, iat };
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = &EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claim, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid(token: &str) -> Result<bool, AppError> {
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());

    decode::<Claims>(&token, &key, &Validation::new(Algorithm::HS256)).map_err(|err| match err
        .kind()
    {
        jsonwebtoken::errors::ErrorKind::InvalidToken => AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized to access this resource",
        ),
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            AppError::new(StatusCode::UNAUTHORIZED, "You token has expired")
        }
        _ => AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    })?;
    Ok(true)
}
