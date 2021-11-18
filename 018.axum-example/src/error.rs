use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use thiserror::Error;

// 使用 thiserror 的派生宏来自定义 WebError 错误类型
#[derive(Debug, Error)]
pub enum WebError {
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error(transparent)]
    UuidError(#[from] uuid::Error),
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    TokioRecvError(#[from] tokio::sync::oneshot::error::RecvError),
    #[error(transparent)]
    AxumExtensionError(#[from] axum::extract::rejection::ExtensionRejection),
    #[error("wrong credentials")]
    WrongCredentials,
    #[error("password doesn't match")]
    WrongPassword,
    #[error("username is already taken")]
    DuplicateUserName,
    #[error("email is already taken")]
    DuplicateUserEmail,
    #[error("phone is already taken")]
    DuplicateUserPhone,
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
}

pub type Result<T> = std::result::Result<T, WebError>;

pub type ApiError = (StatusCode, Json<Value>);
pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl From<WebError> for ApiError {
    fn from(err: WebError) -> Self {
        let status = match err {
            WebError::WrongCredentials => StatusCode::UNAUTHORIZED,
            WebError::ValidationError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let payload = json!({"code": 7, "msg": err.to_string()});
        (status, Json(payload))
    }
}
