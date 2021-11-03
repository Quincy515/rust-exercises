use axum::{
    extract::Extension,
    Json,
    response::{IntoResponse},
    http::StatusCode,
};
use crate::{
    app::model::user::{
        request::user::{
            UserLogin,
            UserRegister,
        },
        response::user::LoginResponse,
    },
    config::databases::Pool,
    util::validate::validate_payload,
};

pub async fn login(
    Json(req): Json<UserLogin>,
    Extension(_pool): Extension<Pool>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    match validate_payload(&req) {
        Ok(_) => (),
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()),
    };
    (StatusCode::OK, "login success".to_string())
}

pub async fn register(
    Json(req): Json<UserRegister>,
    Extension(_pool): Extension<Pool>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    (StatusCode::OK, "注册22")
}