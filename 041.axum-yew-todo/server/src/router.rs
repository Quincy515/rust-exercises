use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    api::{
        hello::hello,
        users::{create_user::create_user, login::login},
    },
    app_state::AppState,
};

pub async fn create_router(app_state: AppState) -> Router {
    let user_routes = Router::new()
        .route("/", post(create_user))
        .route("/login", post(login))
        .with_state(app_state);

    let task_routes = Router::new().route("/", get(|| async {}));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/tasks", task_routes);

    Router::new()
        .route("/", get(hello))
        .nest("/api/v1/", api_routes)
}
