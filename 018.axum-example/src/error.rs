use thiserror::Error;

// 使用 thiserror 的派生宏来自定义 WebError 错误类型
#[derive(Debug, Error, PartialEq)]
pub enum WebError {
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