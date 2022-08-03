#![allow(dead_code)]
use crate::entity::{address, city, customer, prelude::*};
use anyhow::Result;
use sea_orm::{
    DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QuerySelect, RelationTrait,
};

#[derive(Debug, FromQueryResult)]
struct CustomerRes {
    first_name: String,
    last_name: String,
    address: String,
}

///
/// ```sql
/// SELECT `customer`.`first_name`, `customer`.`last_name`, `address`.`address`
/// FROM `customer`
///    INNER JOIN `address` ON `customer`.`address_id` = `address`.`address_id`
///    INNER JOIN `city` ON `address`.`city_id` = `city`.`city_id`;
/// ```
pub async fn joining_three_or_more_tables(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .column(address::Column::Address)
        .inner_join(Address)
        .join(JoinType::InnerJoin, address::Relation::City.def())
        .into_model::<CustomerRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}
