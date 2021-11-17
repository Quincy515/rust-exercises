use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::schema::*;

#[derive(Insertable, Serialize, Deserialize, Debug, Clone, Validate)]
#[table_name = "users"]
pub struct UserRegister {
    #[validate(
        required,
        length(min = 3, max = 20, message = "username is error"),
        custom = "validate_unique_username"
    )]
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    #[validate(required, length(min = 3, message = "password is error"))]
    pub password: Option<String>,
    pub nick_name: Option<String>,
    pub head_img: Option<String>,
    pub authority_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserLogin {
    #[validate(
        required,
        length(min = 3, max = 20, message = "参数错误"),
        custom = "validate_unique_username"
    )]
    pub username: Option<String>,
    #[validate(required, length(min = 3, message = "密码错误"))]
    pub password: Option<String>,
    pub captcha: Option<String>,
    pub captcha_id: Option<String>,
}

fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "custer" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }
    Ok(())
}
