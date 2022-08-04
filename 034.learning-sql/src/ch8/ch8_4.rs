#![allow(dead_code)]
use crate::entity::{film, film_actor, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Expr, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct GroupFilterConditionsRes {
    actor_id: u16,
    rating: String,
    count: i32,
}
/// 要求： 评级为G或PG的电影并且参演电影数大于等于10部的演员。
/// ```
/// SELECT
///   `film_actor`.`actor_id`,
///   `film`.`rating`,
///   COUNT(*) AS `count`
/// FROM
///   `film_actor`
///   INNER JOIN `film` ON `film_actor`.`film_id` = `film`.`film_id`
/// WHERE
///   `film`.`rating` IN ('G', 'PG')
/// GROUP BY
///   `film_actor`.`actor_id`,
///   `film`.`rating`
/// HAVING
///   COUNT(*) > 9
/// ```
pub async fn group_filter_conditions(db: &DatabaseConnection) -> Result<()> {
    let customer = FilmActor::find()
        .select_only()
        .column(film_actor::Column::ActorId)
        .column(film::Column::Rating)
        .column_as(Expr::asterisk().count(), "count")
        .inner_join(Film)
        .filter(film::Column::Rating.is_in(["G", "PG"]))
        .group_by(film_actor::Column::ActorId)
        .group_by(film::Column::Rating)
        .having(Expr::expr(Expr::asterisk().count()).gt(9))
        .into_model::<GroupFilterConditionsRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}
