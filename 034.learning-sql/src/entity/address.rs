//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "address")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub address_id: u16,
    pub address: String,
    pub address2: Option<String>,
    pub district: String,
    pub city_id: u16,
    pub postal_code: Option<String>,
    pub phone: String,
    #[sea_orm(column_type = "Custom(\"GEOMETRY\".to_owned())")]
    pub location: String,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::city::Entity",
        from = "Column::CityId",
        to = "super::city::Column::CityId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    City,
    #[sea_orm(has_many = "super::customer::Entity")]
    Customer,
    #[sea_orm(has_many = "super::staff::Entity")]
    Staff,
    #[sea_orm(has_many = "super::store::Entity")]
    Store,
}

impl Related<super::city::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::City.def()
    }
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::staff::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Staff.def()
    }
}

impl Related<super::store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}