use axum::routing::get;
use axum::{routing::post, Extension, Router};
use sea_orm::DatabaseConnection;

use crate::api::create_task;
use crate::api::custom_json_extractor;
use crate::api::get_one_task;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks/:task_id", get(get_one_task))
        .layer(Extension(database))
}
