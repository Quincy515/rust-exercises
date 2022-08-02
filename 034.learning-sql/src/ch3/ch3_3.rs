#![allow(dead_code)]
use crate::entity::{actor, language, prelude::*};
use anyhow::Result;
use sea_orm::{
    prelude::DateTimeUtc, sea_query::Expr, DatabaseConnection, EntityTrait, FromQueryResult, Order,
    QueryOrder, QuerySelect,
};

pub async fn get_language_list(db: &DatabaseConnection) -> Result<()> {
    let language = Language::find().all(db).await?;
    println!("{:?}", language);
    Ok(())
}

#[derive(Debug, FromQueryResult)]
struct LanguageRes {
    language_id: u8,
    name: String,
    last_update: DateTimeUtc,
}

pub async fn get_language_column(db: &DatabaseConnection) -> Result<()> {
    let language = Language::find()
        .select_only()
        .column(language::Column::LanguageId)
        .column(language::Column::Name)
        .column(language::Column::LastUpdate)
        .into_model::<LanguageRes>()
        .all(db)
        .await?;
    println!("{:?}", language);
    Ok(())
}

#[derive(Debug, FromQueryResult)]
struct LanguageAsRes {
    language_id: u8,
    lang_pi_value: u8,
    language_name: String,
}
pub async fn get_language_column_as(db: &DatabaseConnection) -> Result<()> {
    let language = Language::find()
        .select_only()
        .column(language::Column::LanguageId)
        .column_as(language::Column::Name, "language_name")
        .column_as(
            Expr::col(language::Column::LanguageId).mul(3),
            "lang_pi_value",
        )
        .into_model::<LanguageAsRes>()
        .all(db)
        .await?;
    println!("{:?}", language);
    Ok(())
}

#[derive(Debug, FromQueryResult)]
struct ActorRes {
    actor_id: u16,
}
pub async fn get_actor_id(db: &DatabaseConnection) -> Result<()> {
    let actor = Actor::find()
        .select_only()
        .column(actor::Column::ActorId)
        .order_by(actor::Column::ActorId, Order::Desc)
        .into_model::<ActorRes>()
        .all(db)
        .await?;
    println!("{:?}", actor);
    Ok(())
}
