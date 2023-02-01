use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestUser {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseUser {
    code: i32,
    username: String,
    message: String,
}

pub async fn validate_data(Json(body): Json<RequestUser>) -> Json<ResponseUser> {
    Json(ResponseUser {
        code: 0,
        username: body.username.unwrap_or_default(),
        message: "success".to_string(),
    })
}
