use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use seaorm_axum::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    run(database_uri).await;
}
