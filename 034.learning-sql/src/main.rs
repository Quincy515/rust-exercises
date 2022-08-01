mod ch3;
pub mod entity;

use anyhow::Result;
use sea_orm::{ConnectOptions, Database};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let mut opt = ConnectOptions::new("mysql://root:root1234@localhost:3306/sakila".to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));
    let db = Database::connect(opt).await?;

    // ch3::ch3_1::get_category_list(&db).await?;
    ch3::ch3_3::get_language_list(&db).await?;

    Ok(())
}
