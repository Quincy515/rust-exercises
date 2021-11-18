use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub nick_name: String,
    pub email: String,
    pub phone: String,
    pub head_img: String,
    pub authority_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: 0,
            uuid: uuid::Uuid::new_v4().to_string(),
            username: "".to_string(),
            password: "".to_string(),
            nick_name: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            head_img: "".to_string(),
            authority_id: "".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
