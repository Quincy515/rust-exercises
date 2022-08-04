#![allow(dead_code)]
use crate::entity::{address, prelude::*};
use anyhow::Result;
use sea_orm::{
    sea_query::{Alias, Expr},
    DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct AddressRes {
    addr1: String,
    addr2: String,
    city_id: u16,
}

pub async fn self_join(db: &DatabaseConnection) -> Result<()> {
    let address = Address::find()
        .select_only()
        .column_as(address::Column::Address, "addr1")
        .column_as(
            Expr::tbl(Alias::new("a2"), address::Column::Address).into_simple_expr(),
            "addr2",
        )
        .column(address::Column::CityId)
        .join_as(
            JoinType::InnerJoin,
            address::Entity::belongs_to(address::Entity)
                .from(address::Column::CityId)
                .to(address::Column::CityId)
                .into(),
            Alias::new("a2"),
        )
        // .filter(Condition::all().add(
        //     address::Column::CityId.eq(Expr::tbl(Alias::new("a2"), address::Column::CityId)),
        // ))
        .into_model::<AddressRes>()
        .all(db)
        .await?;
    println!("{:?}", address);
    Ok(())
}
