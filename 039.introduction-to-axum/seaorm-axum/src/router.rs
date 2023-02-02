use axum::{routing::post, Extension, Router};
use sea_orm::DatabaseConnection;

use crate::api::create_task;
use crate::api::custom_json_extractor;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .layer(Extension(database))
}
