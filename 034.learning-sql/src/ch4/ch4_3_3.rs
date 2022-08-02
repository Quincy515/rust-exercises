#![allow(dead_code)]
use crate::entity::{film, prelude::*, sea_orm_active_enums::Rating};
use anyhow::Result;
use sea_orm::{
    sea_query::{Expr, Query},
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct MembershipConditionsRes {
    title: String,
    rating: Option<Rating>,
}

pub async fn membership_conditions(db: &DatabaseConnection) -> Result<()> {
    let customer = Film::find()
        .select_only()
        .column(film::Column::Title)
        .column(film::Column::Rating)
        .filter(
            Condition::any()
                .add(film::Column::Rating.eq("G"))
                .add(film::Column::Rating.eq("PG")),
        )
        .into_model::<MembershipConditionsRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}

pub async fn membership_conditions_in(db: &DatabaseConnection) -> Result<()> {
    let customer = Film::find()
        .select_only()
        .column(film::Column::Title)
        .column(film::Column::Rating)
        .filter(
            Condition::any().add(
                film::Column::Rating.in_subquery(
                    Query::select()
                        .column(film::Column::Rating)
                        .and_where(Expr::col(film::Column::Title).like("%PET%"))
                        .from(Film)
                        .to_owned(),
                ),
            ),
        )
        .into_model::<MembershipConditionsRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}

pub async fn membership_conditions_not_in(db: &DatabaseConnection) -> Result<()> {
    let customer = Film::find()
        .select_only()
        .column(film::Column::Title)
        .column(film::Column::Rating)
        .filter(film::Column::Rating.is_not_in(["PG-13", "R", "NC-17"]))
        .into_model::<MembershipConditionsRes>()
        .all(db)
        .await?;

    println!("{:?}", customer);
    Ok(())
}
