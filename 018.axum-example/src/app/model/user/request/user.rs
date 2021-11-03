use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

use crate::schema::*;

#[derive(diesel::Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name = "users"]
pub struct UserRegister {
    pub username: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub nick_name: String,
    pub head_img: String,
    pub authority_id: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserLogin {
    #[validate(required, length(min = 3, max = 20, message = "参数错误"), custom = "validate_unique_username")]
    pub username: Option<String>,
    #[validate(required, length(min = 3, message = "密码错误"))]
    pub password: Option<String>,
}

fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "custer" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }
    Ok(())
}
