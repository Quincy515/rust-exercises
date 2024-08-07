//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "player_score")]
pub struct Model {
    pub game_id: i32,
    pub player_id: i32,
    pub is_first: i8,
    pub playing_time: i32,
    pub rebound: i32,
    pub rebound_o: i32,
    pub rebound_d: i32,
    pub assist: i32,
    pub score: i32,
    pub steal: i32,
    pub blockshot: i32,
    pub fault: i32,
    pub foul: i32,
    pub shoot_attempts: i32,
    pub shoot_hits: i32,
    pub shoot_3_attempts: i32,
    pub shoot_3_hits: i32,
    pub shoot_p_attempts: i32,
    pub shoot_p_hits: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
