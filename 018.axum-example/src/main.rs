#[macro_use]
extern crate diesel;

use std::env;
use std::net::SocketAddr;
use dotenv::dotenv;

mod config;
mod app;
mod schema;

#[tokio::main]
async fn main(){
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let port = env::var("PORT")
        .expect("server port must be set");
    let port = port.parse::<u16>().unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let pool = config::databases::get_db_pool();
    println!("Start listening on {:?}", addr);

    axum::Server::bind(&addr)
        .serve(config::routes::app(pool).into_make_service())
        .await
        .expect("server failed");
}
