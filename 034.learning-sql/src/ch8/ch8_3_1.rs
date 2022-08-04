#![allow(dead_code)]
use crate::entity::{film_actor, prelude::*};
use anyhow::Result;
use sea_orm::{sea_query::Expr, DatabaseConnection, EntityTrait, FromQueryResult, QuerySelect};

#[derive(Debug, FromQueryResult)]
struct SingleColumnGroupingRes {
    actor_id: u16,
    count: i32,
}

/// 要求： **查找某位演员参演的电影数量**
/// ```
/// SELECT
///   `film_actor`.`actor_id`,
///   COUNT(*) AS `count`
/// FROM
///   `film_actor`
/// GROUP BY
///   `film_actor`.`actor_id`
/// ```
pub async fn single_column_grouping(db: &DatabaseConnection) -> Result<()> {
    let res = FilmActor::find()
        .select_only()
        .column(film_actor::Column::ActorId)
        .column_as(Expr::asterisk().count(), "count")
        .group_by(film_actor::Column::ActorId)
        .into_model::<SingleColumnGroupingRes>()
        .all(db)
        .await?;

    println!("{:?}", res);
    Ok(())
}
