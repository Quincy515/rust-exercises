use serde::{Serialize, Deserialize};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}