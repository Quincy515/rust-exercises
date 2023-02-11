use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    api::{
        hello::hello,
        users::{create_user::create_user, login::login, logout::logout},
    },
    app_state::AppState,
    middleware::require_authentication::require_authentication,
};

pub async fn create_router(app_state: AppState) -> Router {
    let user_routes_auth =
        Router::new()
            .route("/logout", post(logout))
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                require_authentication,
            ));
    let user_routes = Router::new()
        .route("/", post(create_user))
        .route("/login", post(login))
        .merge(user_routes_auth)
        .with_state(app_state);

    let task_routes = Router::new().route("/", get(|| async {}));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/tasks", task_routes);

    Router::new()
        .route("/", get(hello))
        .nest("/api/v1/", api_routes)
}
