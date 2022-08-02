#![allow(dead_code)]
use crate::entity::{address, customer, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::{Alias, Expr},
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct CustomerRes {
    first_name: String,
    last_name: String,
    address: String,
}

pub async fn inner_join(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .column_as(
            Expr::tbl(Alias::new("address"), address::Column::Address).into_simple_expr(),
            "address",
        )
        .inner_join(Address)
        .into_model::<CustomerRes>()
        .all(db)
        .await?;
    println!("{:?}", customer);
    Ok(())
}

pub async fn inner_join_sql92(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .column(address::Column::Address)
        // .column_as(
        //     Expr::tbl(Alias::new("address"), address::Column::Address).into_simple_expr(),
        //     "address",
        // )
        .inner_join(Address)
        .filter(address::Column::PostalCode.eq(52137))
        .into_model::<CustomerRes>()
        .all(db)
        .await?;
    println!("{:?}", customer);
    Ok(())
}
