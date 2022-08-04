#![allow(dead_code)]
use crate::entity::{film, film_actor, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Expr, DatabaseConnection, EntityTrait, FromQueryResult, QueryOrder, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct MulticolumnGroupingRes {
    actor_id: u16,
    count: i32,
}

/// 要求： **找出每位演员参演的各种分级电影（G、PG...）的数量**
/// ```
/// SELECT
///   `film_actor`.`actor_id`,
///   `film`.`rating`,
///   COUNT(*) AS `count`
/// FROM
///   `film_actor`
///   INNER JOIN `film` ON `film_actor`.`film_id` = `film`.`film_id`
/// GROUP BY
///   `film_actor`.`actor_id`,
///   `film`.`rating`
/// ORDER BY
///   `film_actor`.`actor_id` ASC,
///   `film`.`rating` ASC
/// ```
pub async fn multicolumn_grouping(db: &DatabaseConnection) -> Result<()> {
    let res = FilmActor::find()
        .select_only()
        .column(film_actor::Column::ActorId)
        .column(film::Column::Rating)
        .column_as(Expr::asterisk().count(), "count")
        .inner_join(Film)
        .group_by(film_actor::Column::ActorId)
        .group_by(film::Column::Rating)
        .order_by_asc(film_actor::Column::ActorId)
        .order_by_asc(film::Column::Rating)
        .into_model::<MulticolumnGroupingRes>()
        .all(db)
        .await?;

    println!("{:?}", res);
    Ok(())
}
