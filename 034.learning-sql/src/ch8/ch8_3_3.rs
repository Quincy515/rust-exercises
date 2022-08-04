#![allow(dead_code)]
use crate::entity::prelude::*;
use anyhow::Result;
use sea_orm::{
    sea_query::{Expr, Func, Iden},
    DatabaseConnection, EntityTrait, FromQueryResult, QuerySelect,
};

struct Extract;

impl Iden for Extract {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "extract").unwrap();
    }
}

#[derive(Debug, FromQueryResult)]
struct GroupingViaExpressionsRes {
    year: i32,
    how_many: i32,
}
/// 要求： **查询按年份对租借数据进行分组**
/// ```
/// SELECT
///   extract(
///       YEAR
///       FROM
///     `rental_date`
///   ) AS `year`,
///   COUNT(*) AS `how_many`
/// FROM
///   `rental`
/// GROUP BY
///   extract(
///      YEAR
///      FROM FROM
///     `rental_date`
///   )
/// ```
pub async fn grouping_via_expressions(db: &DatabaseConnection) -> Result<()> {
    let res = Rental::find()
        .select_only()
        .column_as(
            Func::cust(Extract).args(vec![
                Expr::expr(Expr::cust("YEAR FROM `rental_date`")),
                // Expr::col(rental::Column::RentalDate),
            ]),
            "year",
        )
        .column_as(Expr::asterisk().count(), "how_many")
        .group_by(Func::cust(Extract).args(vec![
            Expr::expr(Expr::cust("YEAR FROM `rental_date`")),
            // Expr::col(rental::Column::RentalDate),
        ]))
        .into_model::<GroupingViaExpressionsRes>()
        .all(db)
        .await?;

    println!("{:?}", res);
    Ok(())
}
