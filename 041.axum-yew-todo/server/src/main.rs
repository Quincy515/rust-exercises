use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sea_orm::Database;
use server::{app_state::AppState, run};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = dotenv!("DATABASE_URL");
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Error connecting to the databases: {:?}", err);
            std::process::exit(1);
        }
    };
    let app_state = AppState { db };
    run(app_state).await;
}
