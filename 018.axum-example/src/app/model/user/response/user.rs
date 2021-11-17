use serde::{Deserialize, Serialize};
pub struct UserResponse {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub nick_name: String,
    pub head_img: String,
    pub authority_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LoginResponse {
    pub username: Option<String>,
    pub token: Option<String>,
    pub expires_at: Option<String>,
}
