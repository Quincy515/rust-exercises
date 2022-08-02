#![allow(dead_code)]
use crate::entity::{customer, prelude::*};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct CustomerRes {
    first_name: String,
    last_name: String,
}

pub async fn get_customer_empty(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .filter(customer::Column::LastName.eq("SMITH"))
        .into_model::<CustomerRes>()
        .all(db)
        .await?;
    println!("{:?}", customer);
    Ok(())
}

pub async fn get_category_list(db: &DatabaseConnection) -> Result<()> {
    let category = Category::find().all(db).await?;
    println!("{:?}", category);
    Ok(())
}
