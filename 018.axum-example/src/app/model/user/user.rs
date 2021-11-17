use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: u64,
    pub uuid: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub head_img: Option<String>,
    pub authority_id: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
