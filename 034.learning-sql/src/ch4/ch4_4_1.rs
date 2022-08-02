#![allow(dead_code)]
use crate::entity::{prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct NeverEqualNullRes {
    rental_id: i32,
    customer_id: u16,
}
pub async fn never_equal_null(db: &DatabaseConnection) -> Result<()> {
    let rental = Rental::find()
        .select_only()
        .column(rental::Column::RentalId)
        .column(rental::Column::CustomerId)
        .filter(rental::Column::ReturnDate.is_null())
        .into_model::<NeverEqualNullRes>()
        .all(db)
        .await?;

    println!("{:?}", rental);
    Ok(())
}

pub async fn never_equal_not_null(db: &DatabaseConnection) -> Result<()> {
    let rental = Rental::find()
        .select_only()
        .column(rental::Column::RentalId)
        .column(rental::Column::CustomerId)
        .filter(rental::Column::ReturnDate.is_not_null())
        .into_model::<NeverEqualNullRes>()
        .all(db)
        .await?;

    println!("{:?}", rental);
    Ok(())
}

pub async fn never_equal_null_or(db: &DatabaseConnection) -> Result<()> {
    let rental = Rental::find()
        .select_only()
        .column(rental::Column::RentalId)
        .column(rental::Column::CustomerId)
        .filter(
            Condition::any()
                .add(rental::Column::ReturnDate.is_null())
                .add(rental::Column::ReturnDate.not_between("2005-05-01", "2005-09-01")),
        )
        .into_model::<NeverEqualNullRes>()
        .all(db)
        .await?;

    println!("{:?}", rental);
    Ok(())
}
