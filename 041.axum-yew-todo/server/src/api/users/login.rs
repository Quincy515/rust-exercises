use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

use types::user::{RequestCreateUser, ResponseDataUser, ResponseUser};

use crate::{
    queries::user_queries::{find_by_username, save_active_user},
    util::{app_error::AppError, hash::verify_password, jwt::create_token},
};

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(secret): State<String>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let user = find_by_username(&db, request_user.username).await?;

    if !verify_password(&request_user.password, &user.password)? {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "bad username or password",
        ));
    }
    let token = create_token(&secret, user.username.clone())?;
    let mut user = user.into_active_model();
    user.token = Set(Some(token));
    let user = save_active_user(&db, user).await?;

    let response = ResponseUser {
        id: user.id,
        username: user.username,
        token: user.token.unwrap_or_default(),
    };

    Ok(Json(ResponseDataUser { data: response }))
}
