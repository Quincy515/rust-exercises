use axum::{routing::post, Router};
use sea_orm::DatabaseConnection;

use crate::api::custom_json_extractor;

pub async fn create_routes(_database: DatabaseConnection) -> Router {
    Router::new().route("/custom_json_extractor", post(custom_json_extractor))
}
