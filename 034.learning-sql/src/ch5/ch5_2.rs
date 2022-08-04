#![allow(dead_code)]
use crate::entity::{actor, address, customer, film, film_actor, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Alias, ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult,
    JoinType, QueryFilter, QuerySelect, RelationTrait,
};

#[derive(Debug, FromQueryResult)]
struct CustomerRes {
    first_name: String,
    last_name: String,
    address: String,
}

///
/// ```
/// SELECT `customer`.`first_name`, `customer`.`last_name`, `address`.`address`
/// FROM `customer`
///    INNER JOIN `address` ON `customer`.`address_id` = `address`.`address_id`
///    INNER JOIN `city` ON `address`.`city_id` = `city`.`city_id`;
/// ```
pub async fn joining_three_or_more_tables(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .column(address::Column::Address)
        .inner_join(Address)
        .join(JoinType::InnerJoin, address::Relation::City.def())
        .into_model::<CustomerRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}

#[derive(Debug, FromQueryResult)]
struct SameTableTwiceRes {
    title: String,
}

/// ```
/// SELECT
///   `film`.`title`
/// FROM
///   `film`
///   INNER JOIN `film_actor` ON `film`.`film_id` = `film_actor`.`film_id`
///   INNER JOIN `actor` ON `film_actor`.`actor_id` = `actor`.`actor_id`
/// WHERE
///   (
///     `actor`.`first_name` = 'CATE'
///     AND `actor`.`last_name` = 'MCQUEEN'
///   )
///   OR (
///     `actor`.`first_name` = 'CUBA'
///     AND `actor`.`last_name` = 'BIRCH'
///   )
/// ```
pub async fn same_table_twice(db: &DatabaseConnection) -> Result<()> {
    let film = Film::find()
        .select_only()
        .column(film::Column::Title)
        .inner_join(FilmActor)
        .join(JoinType::InnerJoin, film_actor::Relation::Actor.def())
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(actor::Column::FirstName.eq("CATE"))
                        .add(actor::Column::LastName.eq("MCQUEEN")),
                )
                .add(
                    Condition::all()
                        .add(actor::Column::FirstName.eq("CUBA"))
                        .add(actor::Column::LastName.eq("BIRCH")),
                ),
        )
        .into_model::<SameTableTwiceRes>()
        .all(db)
        .await?;
    println!("{:?}", film);
    Ok(())
}

pub async fn same_table_twice_referring(db: &DatabaseConnection) -> Result<()> {
    let film = Film::find()
        .select_only()
        .column(film::Column::Title)
        .join_as(
            JoinType::InnerJoin,
            film::Relation::FilmActor.def(),
            Alias::new("fa1"),
        )
        .join_as(
            JoinType::InnerJoin,
            film_actor::Relation::Actor.def(),
            Alias::new("a1"),
        )
        .join_as(
            JoinType::InnerJoin,
            film::Relation::FilmActor.def(),
            Alias::new("fa2"),
        )
        .join_as(
            JoinType::InnerJoin,
            film_actor::Relation::Actor.def(),
            Alias::new("a2"),
        )
        .filter(
            Condition::all()
                .add(
                    Condition::all()
                        .add(actor::Column::FirstName.eq("CATE"))
                        .add(actor::Column::LastName.eq("MCQUEEN")),
                )
                .add(
                    Condition::all()
                        .add(actor::Column::FirstName.eq("CUBA"))
                        .add(actor::Column::LastName.eq("BIRCH")),
                ),
        )
        .into_model::<SameTableTwiceRes>()
        .all(db)
        .await?;
    println!("{:?}", film);
    Ok(())
}
