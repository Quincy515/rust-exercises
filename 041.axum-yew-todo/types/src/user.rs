use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseDataUser {
    pub data: ResponseUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i32,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    pub username: String,
    pub password: String,
}
