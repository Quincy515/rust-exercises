#![allow(dead_code)]
use entity::post;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use seaorm_demo::establish_connection;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = establish_connection().await?;
    update_post(&db).await?;
    Ok(())
}

async fn update_post(db: &DatabaseConnection) -> Result<(), DbErr> {
    let post = post::Entity::find_by_id(2).one(db).await?;
    let mut post: post::ActiveModel = post.unwrap().into();
    post.title = Set("Update title".to_owned());
    let post: post::Model = post.update(db).await?;
    println!(
        "Post updated for ID: {} with TITLE: {}",
        post.id, post.title
    );
    Ok(())
}
