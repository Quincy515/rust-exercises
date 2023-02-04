use axum::middleware;
use axum::routing::get;
use axum::{routing::post, Extension, Router};
use sea_orm::DatabaseConnection;

use crate::api::atomic_update;
use crate::api::create_task;
use crate::api::create_user;
use crate::api::custom_json_extractor;
use crate::api::delete_task;
use crate::api::get_all_tasks;
use crate::api::get_one_task;
use crate::api::login;
use crate::api::logout;
use crate::api::partial_update;
use crate::guard::guard;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/users/logout", post(logout))
        .route_layer(middleware::from_fn(guard))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task).get(get_all_tasks))
        .route(
            "/tasks/:task_id",
            get(get_one_task)
                .put(atomic_update)
                .patch(partial_update)
                .delete(delete_task),
        )
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .layer(Extension(database))
}
