use dotenvy_macro::dotenv;

mod api;
mod router;

pub async fn run() {
    let app = router::create_router().await;
    let addr = format!("{}:{}", dotenv!("API_URI"), dotenv!("API_PORT"));
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
