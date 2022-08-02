#![allow(dead_code)]
use crate::entity::{customer, prelude::*};
use anyhow::Result;
use sea_orm::{sea_query::Expr, DatabaseConnection, EntityTrait, FromQueryResult, QuerySelect};

#[derive(Debug, FromQueryResult)]
struct CustomerGroupBy {
    first_name: String,
    last_name: String,
    count: i32,
}
pub async fn get_customer_group_by(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .column_as(Expr::asterisk().count(), "count")
        .inner_join(Rental)
        .group_by(customer::Column::FirstName)
        .group_by(customer::Column::LastName)
        .having(Expr::expr(Expr::asterisk().count()).gte(40))
        // .build(DbBackend::MySql)
        // .to_string();
        .into_model::<CustomerGroupBy>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
    Ok(())
}
