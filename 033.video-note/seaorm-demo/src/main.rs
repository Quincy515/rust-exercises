use anyhow::Result;
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<()> {
    let _connection = Database::connect("sqlite://data.sqlite?mode=rwc").await?;
    Ok(())
}
