#![allow(dead_code)]
use entity::{dictation, post};
use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait};
use seaorm_demo::establish_connection;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = establish_connection().await?;
    read_dirtations(&db).await?;
    // read_posts(&db).await?;
    Ok(())
}

async fn read_dirtations(db: &DatabaseConnection) -> Result<(), DbErr> {
    let dictations: Vec<dictation::Model> = dictation::Entity::find().all(db).await?;
    println!("All dirtations ind db:");
    for dictation in dictations {
        println!("ID: {}, TITLE: {:?}", dictation.id, dictation.name);
    }
    Ok(())
}

async fn read_posts(db: &DatabaseConnection) -> Result<(), DbErr> {
    let posts: Vec<post::Model> = post::Entity::find().all(db).await?;
    println!("All posts ind db:");
    for post in posts {
        println!("ID: {}, TITLE: {}", post.id, post.title);
    }
    Ok(())
}
