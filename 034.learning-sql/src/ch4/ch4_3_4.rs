#![allow(dead_code)]
use crate::entity::{customer, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::Expr, ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult,
    QueryFilter, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct MatchingConditionsRes {
    last_name: String,
    first_name: String,
}

pub async fn matching_conditions(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::LastName)
        .column(customer::Column::FirstName)
        .filter(customer::Column::LastName.starts_with("Q"))
        .into_model::<MatchingConditionsRes>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
    Ok(())
}

pub async fn matching_conditions_mul(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::LastName)
        .column(customer::Column::FirstName)
        .filter(
            Condition::any()
                .add(customer::Column::LastName.starts_with("Q"))
                .add(customer::Column::LastName.starts_with("Y")),
        )
        .into_model::<MatchingConditionsRes>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
    Ok(())
}

pub async fn matching_conditions_regexp(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::LastName)
        .column(customer::Column::FirstName)
        .filter(Expr::cust(r#"`last_name` REGEXP '^[QY]'"#))
        .into_model::<MatchingConditionsRes>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
    Ok(())
}
