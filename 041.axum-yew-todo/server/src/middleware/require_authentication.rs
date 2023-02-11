use axum::{
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::util::{app_error::AppError, jwt::validate_token};
use entity::{prelude::*, users};

pub async fn require_authentication<B>(
    State(db): State<DatabaseConnection>,
    State(secret): State<String>,
    headers: HeaderMap,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let token = if let Some(token) = headers.get("x-token") {
        token.to_str().map_err(|err| {
            eprintln!("Error extracting token from headers: {err:?}");
            AppError::new(StatusCode::BAD_REQUEST, err.to_string())
        })?
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Missing authentication token",
        ));
    };

    validate_token(&secret, token)?;

    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token.to_owned())))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting user by token: {err:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a problem getting your account",
            )
        })?;

    if let Some(user) = user {
        request.extensions_mut().insert(user);
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized for this",
        ));
    };
    Ok(next.run(request).await)
}
