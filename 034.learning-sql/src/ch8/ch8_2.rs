#![allow(dead_code)]
use crate::entity::{payment, prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    prelude::Decimal,
    sea_query::{Expr, Func, Iden},
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct PaymentRes {
    max_amt: Decimal,
    min_amt: Decimal,
    tot_amt: Decimal,
    num_payments: i32,
}

/// 要求：常见的聚合函数来分析电影租借付款数据
/// ```
/// SELECT
///   MAX(`payment`.`amount`) AS `max_amt`,
///   MIN(`payment`.`amount`) AS `min_amt`,
///   SUM(`payment`.`amount`) AS `tot_amt`,
///   COUNT(*) AS `num_payments`
/// FROM
///   `payment`
/// ```
pub async fn aggregate_functions(db: &DatabaseConnection) -> Result<()> {
    let payment = Payment::find()
        .select_only()
        .column_as(payment::Column::Amount.max(), "max_amt")
        .column_as(payment::Column::Amount.min(), "min_amt")
        .column_as(payment::Column::Amount.sum(), "tot_amt")
        .column_as(Expr::asterisk().count(), "num_payments")
        .into_model::<PaymentRes>()
        .all(db)
        .await?;
    println!("{:?}", payment);
    Ok(())
}

#[derive(Debug, FromQueryResult)]
struct CustomerPayment {
    customer_id: u16,
    max_amt: Decimal,
    min_amt: Decimal,
    tot_amt: Decimal,
    num_payments: i32,
}

/// 要求： 每位顾客的电影租借付款数据
/// ```
/// SELECT
///   `payment`.`customer_id`,
///   MAX(`payment`.`amount`) AS `max_amt`,
///   MIN(`payment`.`amount`) AS `min_amt`,
///   SUM(`payment`.`amount`) AS `tot_amt`,
///   COUNT(*) AS `num_payments`
/// FROM
///   `payment`
/// GROUP BY
///   `payment`.`customer_id`
/// ```
pub async fn aggregate_functions_customer(db: &DatabaseConnection) -> Result<()> {
    let payment = Payment::find()
        .select_only()
        .column(payment::Column::CustomerId)
        .column_as(payment::Column::Amount.max(), "max_amt")
        .column_as(payment::Column::Amount.min(), "min_amt")
        .column_as(payment::Column::Amount.sum(), "tot_amt")
        .column_as(Expr::asterisk().count(), "num_payments")
        .group_by(payment::Column::CustomerId)
        .into_model::<CustomerPayment>()
        .all(db)
        .await?;
    println!("{:?}", payment);
    Ok(())
}

/// 在 `sea_orm` 中结合使用 `sea_query` 方法
/// ```
/// SELECT
///   COUNT(customer_id) num_rows,
///   COUNT(DISTINCT customer_id) num_customers
/// FROM
///   payment;
/// ```
async fn counting_distinct_values(db: &DatabaseConnection) -> Result<()> {
    let count = Payment::find()
        .select_only()
        .column_as(payment::Column::CustomerId.count(), "num_rows")
        .column_as(
            Expr::col(payment::Column::CustomerId).count(),
            "num_customers",
        )
        .all(db)
        .await?;
    println!("{:?}", count);
    Ok(())
}

struct Datediff;

impl Iden for Datediff {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "datediff").unwrap();
    }
}

#[derive(Debug, FromQueryResult)]
struct UsingExpressionsRes {
    max: i32,
}

/// 要求： **找出一部电影从被租借到后来归还之间相隔的最大天数**
/// ```
/// SELECT
///   MAX(
///      datediff(`return_date`, `rental_date`)
///   ) AS `max`
/// FROM
///   `rental`
/// ```
pub async fn using_expressions(db: &DatabaseConnection) -> Result<()> {
    let rental_date = Rental::find()
        .select_only()
        .column_as(
            Expr::expr(Func::cust(Datediff).args(vec![
                Expr::col(rental::Column::ReturnDate),
                Expr::col(rental::Column::RentalDate),
            ]))
            .max(),
            "max",
        )
        .into_model::<UsingExpressionsRes>()
        .all(db)
        .await?;
    println!("{:?}", rental_date);
    Ok(())
}
