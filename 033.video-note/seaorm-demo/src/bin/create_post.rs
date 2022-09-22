use entity::post;
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{Database, DbConn, Set, ActiveModelTrait};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = establish_connection().await?;
    let post = post::ActiveModel {
        title: Set(String::from("Amazint title 1")),
        text: Set(String::from("Lorem ipsum dolor sit amet.")),
        ..Default::default()
    };
    let post: post::Model = post.insert(&db).await?;
    println!("Post created with ID: {}, TITLE: {}", post.id, post.title);
    Ok(())
}

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
