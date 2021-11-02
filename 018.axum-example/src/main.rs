use std::env;
use std::net::SocketAddr;

use anyhow::Result;
use dotenv::dotenv;

mod config;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let port = env::var("PORT")
        .expect("server port must be set");
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let pool = config::databases::get_db_pool();

    axum::Server::bind(&addr)
        .serve(config::routes::app(pool).into_make_service())
        .await?;
    Ok(())
}
