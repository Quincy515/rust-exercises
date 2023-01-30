pub mod hello_world;

use axum::{Router, routing::get};
use hello_world::hello_world;

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_world))
}