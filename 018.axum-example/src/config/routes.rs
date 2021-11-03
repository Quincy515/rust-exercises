use axum::{
    routing::{get, post},
    AddExtensionLayer,
    Router,
    handler::Handler,
    http::{StatusCode, Uri},
    response::{IntoResponse},
};
use crate::{
    app::api::user::user_auth::{
        login,
        register,
    },
    config::databases::Pool,
};

pub fn app(pool: Pool) -> Router {
    Router::new()
        .route("/", get(|| async { "Welcome to use axum" }))
        .nest("/api", api())
        .layer(AddExtensionLayer::new(pool))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .fallback(fallback.into_service())
}

pub fn api() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
