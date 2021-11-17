use axum::{extract::Extension, Json};
use serde_json::{json, Value};

use crate::{
    app::{
        model::{
            common::response::response::Response,
            user::{request::user::UserLogin, response::user::LoginResponse},
        },
        service::system::sys_user::UserService,
    },
    config::databases::Pool,
    error::ApiResult,
    util::{jwt, validate::validate_payload},
    WebError,
};

pub async fn login(
    Json(req): Json<UserLogin>,
    Extension(_pool): Extension<Pool>,
) -> ApiResult<Json<Value>> {
    validate_payload(&req)?;
    let user = UserService::login(req, &_pool)
        .await
        .map_err(|_| WebError::WrongCredentials)?;
    let token = jwt::sign(user.uuid.unwrap())?;
    let res = Response::<LoginResponse>::ok_with_msg("Login Successful".to_string());
    Ok(Json(json!(res)))
}
