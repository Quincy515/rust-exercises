use entity::post;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use seaorm_demo::establish_connection;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = establish_connection().await?;
    let post = post::Entity::find_by_id(2).one(&db).await?;
    let mut post: post::ActiveModel = post.unwrap().into();
    post.title = Set("Update title".to_owned());
    let post: post::Model = post.update(&db).await?;
    println!(
        "Post updated for ID: {} with TITLE: {}",
        post.id, post.title
    );
    Ok(())
}
