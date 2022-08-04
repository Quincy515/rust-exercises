#![allow(dead_code)]
use crate::entity::{city, country, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Query, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
pub struct NoncorrelatedSubqueriesRes {
    city_id: u16,
    city: String,
}

/// 要求： **查询返回所有不在印度的城市**
/// ```
/// SELECT
///   `city`.`city_id`,
///   `city`.`city`
/// FROM
///   `city`
/// WHERE
///   `city`.`country_id` NOT IN (
///     SELECT
///         `country_id`
///     FROM
///         `country`
///     WHERE
///         `country`.`country` = 'India'
///   )
/// ```
pub async fn noncorrelated_subqueries(db: &DatabaseConnection) -> Result<()> {
    let res = City::find()
        .select_only()
        .column(city::Column::CityId)
        .column(city::Column::City)
        .filter(
            city::Column::CountryId.not_in_subquery(
                Query::select()
                    .column(country::Column::CountryId)
                    .from(Country)
                    .and_where(country::Column::Country.eq("India"))
                    .to_owned(),
            ),
        )
        .into_model::<NoncorrelatedSubqueriesRes>()
        .all(db)
        .await?;
    println!("{:?}", res);
    Ok(())
}
