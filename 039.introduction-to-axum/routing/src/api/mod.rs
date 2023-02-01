pub mod always_errors;
pub mod custom_middleware;
pub mod get_json;
pub mod hello_world;
pub mod middleware_message;
pub mod mirror_body_json;
pub mod mirror_body_string;
pub mod mirror_custom_header;
pub mod mirror_user_agent;
pub mod path_variable;
pub mod query_params;
pub mod returns_201;
pub mod set_custom_middleware;
pub mod validate_data;

use always_errors::always_errors;
use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use custom_middleware::custom_middleware;
use get_json::get_json;
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variable::{hard_coded_path, path_variable};
use query_params::query_params;
use returns_201::returns_201;
use set_custom_middleware::set_custom_middleware;
use tower_http::cors::{Any, CorsLayer};
use validate_data::validate_data;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    Router::new()
        .route("/custom_middleware", get(custom_middleware))
        .route_layer(middleware::from_fn(set_custom_middleware))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
        .route("/always_errors", get(always_errors))
        .route("/returns_201", get(returns_201))
        .route("/get_json", get(get_json))
        .route("/validate_data", post(validate_data))
}
