#![allow(dead_code)]
use crate::entity::{film, prelude::*, sea_orm_active_enums::Rating};
use anyhow::Result;
use sea_orm::{
    sea_query::Expr, Condition, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct RatingFilmRes {
    title: String,
}

pub async fn get_rating_film_where(db: &DatabaseConnection) -> Result<()> {
    let rating = Film::find()
        .select_only()
        .column(film::Column::Title)
        .filter(
            Condition::any()
                .add(Expr::col(film::Column::Rating).eq("G"))
                .add(Expr::col(film::Column::RentalDuration).gte(7)),
        )
        .into_model::<RatingFilmRes>()
        .all(db)
        .await?;
    println!("{:?}", rating);
    Ok(())
}

#[derive(Debug, FromQueryResult)]
struct RatingFilmAndOrRes {
    title: String,
    rating: Option<Rating>,
    rental_duration: u8,
}

pub async fn get_rating_film_mul_where(db: &DatabaseConnection) -> Result<()> {
    let rating = Film::find()
        .select_only()
        .column(film::Column::Title)
        .column(film::Column::Rating)
        .column(film::Column::RentalDuration)
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(Expr::col(film::Column::Rating).eq("G"))
                        .add(Expr::col(film::Column::RentalDuration).gte(7)),
                )
                .add(
                    Condition::all()
                        .add(Expr::col(film::Column::Rating).eq("PG-13"))
                        .add(Expr::col(film::Column::RentalDuration).lt(4)),
                ),
        )
        .into_model::<RatingFilmAndOrRes>()
        .all(db)
        .await?;
    println!("{:?}", rating);
    Ok(())
}
