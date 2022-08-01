use crate::entity::{language, prelude::*};
use anyhow::Result;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn get_language_list(db: &DatabaseConnection) -> Result<()> {
    let language = Language::find().all(db).await?;
    println!("{:?}", language);
    Ok(())
}
