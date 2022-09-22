use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let database_url = "sqlite://data.sqlite?mode=rwc";
    let db = Database::connect(database_url)
        .await
        .expect("Failed to setup the database");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    Ok(db)
}
