// Handler or Endpoint
async fn hello() -> &'static str {
    "<h1> Hello World!</h>"
}

// #[tokio::main]
// async fn main() {
//     let app = axumlike::app()
//         // .layer(tower_http::authenticate)
//         .route("/", get(hello))
//         // .route("/users/:id",any(users))
//         ;
//     axumlike::start(([127, 0, 0, 1], 3000))
//         .serve(app.into_make_service())
//         .await.unwrap();
// }


use std::net::SocketAddr;
use axumlike::{handler::get, Router};

use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;
    // build our application with a route
    let app = Router::new().route("/", get(hello));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!("listening on {}", addr);

    // tracing 宏自己的语法 % 表示实现 display，？表示实现 debug
    info!(%addr, "listening on");

    axumlike::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.unwrap();

    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}