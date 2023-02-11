use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sea_orm::Database;
use server::{app_state::AppState, run};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url: &str = dotenv!("DATABASE_URL");
    let jwt_secret: String = dotenv!("JWT_SECRET").to_owned();
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Error connecting to the databases: {:?}", err);
            std::process::exit(1);
        }
    };
    let app_state = AppState { db, jwt_secret };
    run(app_state).await;
}
