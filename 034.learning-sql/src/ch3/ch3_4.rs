#![allow(dead_code)]
use crate::entity::{customer, prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    entity::prelude::*,
    sea_query::{Alias, Expr},
    Condition, DatabaseConnection, EntityTrait, FromQueryResult, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct CustomerRes {
    full_name: String,
}
pub async fn subquery_customer_list(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column_as(
            customer::Column::LastName,
            // Expr::col(customer::Column::LastName)
            //     .concat(Expr::val(" "))
            //     .concat(Expr::col(customer::Column::FirstName)),
            "full_name",
        )
        .into_model::<CustomerRes>()
        .all(db)
        .await?;
    println!("{:?}", customer);
    Ok(())
}

#[derive(Debug, FromQueryResult)]
struct CustomerTableRes {
    first_name: String,
    last_name: String,
    rental_time: DateTime,
}
pub async fn subquery_customer_table_link(db: &DatabaseConnection) -> Result<()> {
    let customer =
        Customer::find()
            .select_only()
            .column(customer::Column::FirstName)
            .column(customer::Column::LastName)
            .column_as(
                Expr::tbl(Alias::new("rental"), rental::Column::RentalDate).into_simple_expr(),
                "rental_time",
            )
            .inner_join(Rental)
            .filter(Condition::all().add(
                Expr::col(rental::Column::RentalDate).between::<_>("2005-06-14", "2005-06-16"),
            ))
            // .build(DbBackend::MySql)
            // .to_string();
            .into_model::<CustomerTableRes>()
            .all(db)
            .await?;
    println!("{:?}", customer);
    Ok(())
}
