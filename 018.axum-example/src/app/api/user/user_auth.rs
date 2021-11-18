use crate::{
    app::{
        model::user::{
            request::user::{UserLogin, UserRegister},
            user::User,
        },
        service::user::user_auth::UserService,
    },
    config::databases::Pool,
    util::validate::validate_payload,
};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};

pub async fn login(
    Json(req): Json<UserLogin>,
    Extension(_pool): Extension<Pool>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    match validate_payload(&req) {
        Ok(_) => {
            let user = UserService::find_username(&_pool, req.username.unwrap().as_str());
            return (StatusCode::OK, Json(user.unwrap()));
        }
        Err(_) => {
            return (StatusCode::OK, Json(User::new()));
        }
    };
}

pub async fn register(
    Json(req): Json<UserRegister>,
    Extension(_pool): Extension<Pool>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    if let Some(_) = validate_payload(&req).ok() {
        let user = UserService::create(&_pool, req);
        return (StatusCode::OK, Json(user.unwrap()));
    }
    (StatusCode::OK, Json(User::new()))
}
