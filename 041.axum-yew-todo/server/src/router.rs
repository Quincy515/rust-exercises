use axum::{routing::get, Router};

use crate::api::hello::hello;

pub async fn create_router() -> Router {
    Router::new().route("/", get(hello))
}
