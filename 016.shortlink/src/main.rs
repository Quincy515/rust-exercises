mod handlers;
mod dto;
mod models;

use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;
use axum::{AddExtensionLayer, BoxError, Router};
use axum::handler::{get, post};
use axum::http::StatusCode;
use sqlx::mysql::MySqlPoolOptions;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root1234@localhost/shorten_db")
        .await?;

    let app = Router::new()
        .route("/", get(handlers::hello))
        .route("/api/create_shortlink", post(handlers::create_shortlink))
        .route("/api/delete_shortlink", post(handlers::delete_shortlink))
        // .router("/api/get_shortlink", get(handlers::get_shortlink))
        .layer(
            ServiceBuilder::new()
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(AddExtensionLayer::new(pool))
                .into_inner()
        )
        .handle_error(|error: BoxError| {
            let result = if error.is::<tower::timeout::error::Elapsed>() {
                Ok(StatusCode::REQUEST_TIMEOUT)
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("{}", error),
                ))
            };
            Ok::<_, Infallible>(result)
        })
        .check_infallible();

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}