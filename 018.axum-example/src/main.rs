#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

use std::net::SocketAddr;

mod app;
mod config;
mod error;
mod schema;
mod util;

use clap::Parser;
pub use error::WebError;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();

    let config = config::env::ServerConfig::parse();
    let addr = SocketAddr::from((config.host, config.port));
    let pool = config::databases::get_db_pool();
    info!("Start listening on {:?}", addr);

    axum::Server::bind(&addr)
        .serve(config::routes::app(pool).into_make_service())
        .await
        .expect("server failed");
}
