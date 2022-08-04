#![allow(dead_code)]
use crate::entity::{customer, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Query, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
pub struct SelectResult {
    customer_id: u16,
    first_name: String,
    last_name: String,
}

/// 要求： **在单个查询中检索ID值最大的客户信息**
/// ```
/// SELECT
///   `customer`.`customer_id`,
///   `customer`.`first_name`,
///   `customer`.`last_name`
/// FROM
///   `customer`
/// WHERE
///   `customer`.`customer_id` IN (
///     SELECT
///         MAX(`customer`.`customer_id`)
///     FROM
///         `customer`
///   )
///
/// ```
pub async fn what_is_a_subquery(db: &DatabaseConnection) -> Result<()> {
    let res = Customer::find()
        .select_only()
        .column(customer::Column::CustomerId)
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .filter(
            customer::Column::CustomerId.in_subquery(
                Query::select()
                    .expr(customer::Column::CustomerId.max())
                    .from(Customer)
                    .to_owned(),
            ),
        )
        .into_model::<SelectResult>()
        .all(db)
        .await?;
    println!("{:?}", res);
    Ok(())
}
