use dotenvy::dotenv;
use server::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    run().await;
}
