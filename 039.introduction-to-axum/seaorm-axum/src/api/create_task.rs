use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json, TypedHeader,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

use crate::databases::{prelude::*, tasks, users};

#[derive(Deserialize)]
pub struct RequestTask {
    priority: Option<String>,
    title: String,
    description: Option<String>,
}

pub async fn create_task(
    // Extension(database): Extension<DatabaseConnection>,
    State(database): State<DatabaseConnection>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let token = authorization.token();
    let user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let result = new_task.save(&database).await.unwrap();
    dbg!(result);
    Ok(())
}
