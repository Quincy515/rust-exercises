#![allow(dead_code)]
use crate::entity::{prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    entity::prelude::*, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct RangeConditionsRes {
    customer_id: u16,
    rental_date: DateTime,
}

pub async fn range_conditions(db: &DatabaseConnection) -> Result<()> {
    let customer = Rental::find()
        .select_only()
        .column(rental::Column::CustomerId)
        .column(rental::Column::RentalDate)
        .filter(rental::Column::RentalDate.lt("2005-05-25"))
        .into_model::<RangeConditionsRes>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
    Ok(())
}
