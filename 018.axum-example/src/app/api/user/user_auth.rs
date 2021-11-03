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
};

pub async fn login(
    Json(req): Json<UserLogin>,
    Extension(_pool): Extension<Pool>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    (StatusCode::OK, "登录")
}

pub async fn register(
    Json(req): Json<UserRegister>,
    Extension(_pool): Extension<Pool>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    (StatusCode::OK, "注册22")
}