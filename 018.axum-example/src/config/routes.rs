use crate::{
    app::api::{
        system::{sys_captcha::captcha, sys_user::login as sys_login},
        user::user_auth::{login, register},
    },
    config::databases::Pool,
};
use axum::{
    handler::Handler,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::{get, post},
    AddExtensionLayer, Router,
};

pub fn app(pool: Pool) -> Router {
    Router::new()
        .route("/", get(|| async { "Welcome to use axum" }))
        .nest("/api", api())
        .nest("/base", base())
        .layer(AddExtensionLayer::new(pool))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .fallback(fallback.into_service())
}

pub fn api() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/captcha", post(captcha))
}

pub fn base() -> Router {
    Router::new()
        .route("/login", post(sys_login))
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
