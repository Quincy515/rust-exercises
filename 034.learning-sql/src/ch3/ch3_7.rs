#![allow(dead_code)]

use crate::entity::{customer, prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    entity::prelude::*,
    sea_query::{Alias, Expr},
    DatabaseConnection, EntityTrait, FromQueryResult, JoinType, Order, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};

#[derive(Debug, FromQueryResult)]
struct CustomerOrderBy {
    first_name: String,
    last_name: String,
    rental_time: DateTime,
}

pub async fn get_customer_order_by(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .column_as(
            Expr::tbl(Alias::new("rental"), rental::Column::RentalDate).into_simple_expr(),
            "rental_time",
        )
        .join_rev(JoinType::InnerJoin, rental::Relation::Customer.def())
        .filter(Expr::col(rental::Column::RentalDate).between::<_>("2005-06-14", "2005-06-15"))
        .order_by_asc(customer::Column::LastName)
        .order_by(customer::Column::FirstName, Order::Asc)
        .into_model::<CustomerOrderBy>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
    Ok(())
}
