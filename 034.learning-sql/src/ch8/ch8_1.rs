#![allow(dead_code)]
use crate::entity::{prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    sea_query::Expr, DatabaseConnection, EntityTrait, FromQueryResult, QueryOrder, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct CustomerGropuByRes {
    customer_id: u16,
    count: i32,
}
/// 要想知道每位客户租借了多少部电影，可以在select子句中使用聚合函数，以统计每组有多少行：
/// ```
/// SELECT
///   `rental`.`customer_id`,
///   COUNT(*) AS `count`
/// FROM
///   `rental`
/// GROUP BY
///   `rental`.`customer_id`
/// ```
pub async fn group_by(db: &DatabaseConnection) -> Result<()> {
    let customer = Rental::find()
        .select_only()
        .column(rental::Column::CustomerId)
        .column_as(Expr::asterisk().count(), "count")
        .group_by(rental::Column::CustomerId)
        .into_model::<CustomerGropuByRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}

/// 要求： **确定哪位客户借的电影最多**
/// ```
/// SELECT
///   `rental`.`customer_id`,
///   COUNT(*) AS `count`
/// FROM
///   `rental`
/// GROUP BY
///   `rental`.`customer_id`
/// ORDER BY
///   COUNT(*) DESC
/// ```
pub async fn group_by_order_by(db: &DatabaseConnection) -> Result<()> {
    let customer = Rental::find()
        .select_only()
        .column(rental::Column::CustomerId)
        .column_as(Expr::asterisk().count(), "count")
        .group_by(rental::Column::CustomerId)
        .order_by_desc(Expr::asterisk().count())
        .into_model::<CustomerGropuByRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}

/// 要求： 租借电影数量为40或多于40部的客户
/// ```
/// SELECT 
///   `rental`.`customer_id`, 
///   COUNT(*) AS `count` 
/// FROM 
///   `rental` 
/// GROUP BY 
///   `rental`.`customer_id` 
/// HAVING 
///   COUNT(*) >= 40
/// ```
pub async fn group_by_order_by_having(db: &DatabaseConnection) -> Result<()> {
    let customer = Rental::find()
        .select_only()
        .column(rental::Column::CustomerId)
        .column_as(Expr::asterisk().count(), "count")
        .group_by(rental::Column::CustomerId)
        .having(Expr::expr(Expr::asterisk().count()).gte(40))
        .into_model::<CustomerGropuByRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}
