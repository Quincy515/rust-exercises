use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, diesel::Queryable)]
pub struct User {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub nick_name: String,
    pub email: String,
    pub phone: String,
    pub head_img: String,
    pub authority_id: String,
}