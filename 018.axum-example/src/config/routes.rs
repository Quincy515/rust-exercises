use axum::handler::get;
use axum::{AddExtensionLayer, Router};
use axum::routing::BoxRoute;

use crate::config::databases::Pool;

pub fn app(pool: Pool) -> Router<BoxRoute>{
    Router::new()
        .route("/", get(|| async {"Welcome to use axum"}))
        .nest("/api", api())
        .layer(AddExtensionLayer::new(pool))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .boxed()
}

pub fn api() -> Router<BoxRoute>{
    Router::new()
        .route("/not_found", get(|| async {"not found"}))
        .boxed()
}
