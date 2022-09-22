#![allow(dead_code)]
use entity::{dictation, post};
use migration::DbErr;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use seaorm_demo::establish_connection;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = establish_connection().await?;
    create_dictation(&db).await?;
    // create_post(&db).await?;
    Ok(())
}

async fn create_dictation(db: &DatabaseConnection) -> Result<(), DbErr> {
    let dictation = dictation::ActiveModel {
        name: Set(Some(String::from("TestPeper 1"))),
        ..Default::default()
    };
    let dictation: dictation::Model = dictation.insert(db).await?;
    println!(
        "dictation created with ID: {}, NAME: {:?}",
        dictation.id, dictation.name
    );
    Ok(())
}

async fn create_post(db: &DatabaseConnection) -> Result<(), DbErr> {
    let post = post::ActiveModel {
        title: Set(String::from("Amazint title 1")),
        text: Set(String::from("Lorem ipsum dolor sit amet.")),
        ..Default::default()
    };
    let post: post::Model = post.insert(db).await?;
    println!("Post created with ID: {}, TITLE: {}", post.id, post.title);
    Ok(())
}
