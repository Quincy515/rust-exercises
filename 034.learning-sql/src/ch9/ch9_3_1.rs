#![allow(dead_code)]
use crate::entity::{address, city, country, customer, payment, prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    sea_query::{Expr, Query},
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, JoinType,
    QueryFilter, QuerySelect, RelationTrait,
};

#[derive(Debug, FromQueryResult)]
pub struct MultipleRowSingleColumnSubqueriesRes {
    city_id: u16,
    city: String,
}

/// 要求： **查找位于Canada或Mexico的所有城市**
/// ```
/// SELECT
///   `city`.`city_id`,
///   `city`.`city`
/// FROM
///   `city`
/// WHERE
///   `city`.`country_id` IN (
///     SELECT
///       `country_id`
///     FROM
///       `country`
///     WHERE
///       `country`.`country` IN ('Canada', 'Mexico')
///   )
/// ```
pub async fn multiple_row_single_column_subqueries(db: &DatabaseConnection) -> Result<()> {
    let res = City::find()
        .select_only()
        .column(city::Column::CityId)
        .column(city::Column::City)
        .filter(
            city::Column::CountryId.in_subquery(
                Query::select()
                    .column(country::Column::CountryId)
                    .from(Country)
                    .and_where(country::Column::Country.is_in(["Canada", "Mexico"]))
                    .to_owned(),
            ),
        )
        .into_model::<MultipleRowSingleColumnSubqueriesRes>()
        .all(db)
        .await?;
    println!("{:?}", res);
    Ok(())
}

#[derive(Debug, FromQueryResult)]
struct AllOperatorRes {
    first_name: String,
    last_name: String,
}

/// 要求： **查询搜索所有从未获得过免费电影租借的客户**
/// ```
/// SELECT
///   `customer`.`first_name`,
///   `customer`.`last_name`
/// FROM
///   `customer`
/// WHERE
///   `customer`.`customer_id` NOT IN (
///     SELECT
///       `customer_id`
///     FROM
///       `payment`
///     WHERE
///       `payment`.`amount` = 0
///   )
/// ```
pub async fn all_operator(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .filter(
            customer::Column::CustomerId.not_in_subquery(
                Query::select()
                    .column(payment::Column::CustomerId)
                    .from(Payment)
                    .and_where(payment::Column::Amount.eq(0))
                    .to_owned(),
            ),
        )
        .into_model::<AllOperatorRes>()
        .all(db)
        .await?;
    println!("{:?}", customer);
    Ok(())
}

#[derive(Debug, FromQueryResult)]
struct AllOperatorEqualInRes {
    customer_id: u16,
    count: i32,
}
pub async fn all_operator_equal_in(db: &DatabaseConnection) -> Result<()> {
    let res = 
    // Rental::find()
    //     .select_only()
    //     .column(rental::Column::CustomerId)
    //     .column_as(Expr::asterisk().count(), "count")
    //     .group_by(rental::Column::CustomerId)
        // .having(
        //     Expr::expr(Expr::asterisk().count()).in_subquery(
        //         Query::select()
        //             .column((Rental, Expr::asterisk().count()))
        //             .from(Rental)
        //             .to_owned(),
        //     ),
        // )
        Rental::find()
                    .select_only()
                    .column_as(Expr::asterisk().count(), "count")
                    .inner_join(Customer)
                    .join(JoinType::InnerJoin, customer::Relation::Address.def())
                    .join(JoinType::InnerJoin, address::Relation::City.def())
                    .join(JoinType::InnerJoin, city::Relation::Country.def())
                    .filter(country::Column::Country.is_in(["United States", "Mexico", "Canada"]))
                    .group_by(rental::Column::CustomerId)
        .into_model::<AllOperatorEqualInRes>()
        .all(db)
        .await?;

    println!("{:?}", res);
    Ok(())
}
