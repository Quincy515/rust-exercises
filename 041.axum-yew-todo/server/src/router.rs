use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};

use crate::{
    api::{
        hello::hello,
        tasks::{
            create_task::create_task,
            delete_task::soft_delete_task,
            get_all_tasks::get_all_tasks,
            get_one_task::get_one_task,
            update_tasks::{mark_completed, mark_uncompleted, update_task},
        },
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
        .merge(user_routes_auth);

    let task_routes = Router::new()
        .route("/", post(create_task))
        .route("/", get(get_all_tasks))
        .route("/:task_id", get(get_one_task))
        .route("/:task_id/completed", put(mark_completed))
        .route("/:task_id/uncompleted", put(mark_uncompleted))
        .route("/:task_id", patch(update_task))
        .route("/:task_id", delete(soft_delete_task))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/tasks", task_routes);

    Router::new()
        .route("/", get(hello))
        .nest("/api/v1/", api_routes)
        .with_state(app_state)
}
