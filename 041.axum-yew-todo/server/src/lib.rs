use dotenvy_macro::dotenv;

use app_state::AppState;

mod api;
pub mod app_state;
mod middleware;
mod router;
mod util;

pub async fn run(app_state: AppState) {
    let app = router::create_router(app_state).await;
    let addr = format!("{}:{}", dotenv!("API_URI"), dotenv!("API_PORT"));
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
