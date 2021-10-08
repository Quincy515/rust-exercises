


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

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(hello));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axumlike::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.unwrap();
}
