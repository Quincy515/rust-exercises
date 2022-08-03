#![allow(dead_code)]
use crate::entity::{actor, customer, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::{MysqlQueryBuilder, Query, UnionType},
    DatabaseConnection, EntityTrait, FromQueryResult, QuerySelect, QueryTrait,
};

#[derive(Debug, FromQueryResult)]
struct CustomerRes {
    first_name: String,
    last_name: String,
}
pub async fn union_all(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::FirstName)
        .column(customer::Column::LastName)
        .into_query()
        .union(
            UnionType::All,
            Query::select()
                .columns([actor::Column::FirstName, actor::Column::LastName])
                .from(Actor)
                .to_owned(),
        )
        .to_owned()
        .to_string(MysqlQueryBuilder);
    // .into_model::<CustomerRes>()
    // .all(db)
    // .await?;
    println!("{:?}", customer);
    Ok(())
}
