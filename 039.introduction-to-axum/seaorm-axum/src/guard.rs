use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::databases::users;
use crate::util::app_error::AppError;
use crate::{databases::prelude::*, util::jwt::is_valid};

pub async fn guard<T>(mut request: Request<T>, next: Next<T>) -> Result<Response, AppError> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing Bearer token"))?
        .token()
        .to_owned();
    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or_else(|| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))?;
    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token.clone())))
        .one(database)
        .await
        .map_err(|err| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    is_valid(&token)?;
    let Some(user) = user else {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "You are not authorized"));
    };
    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
