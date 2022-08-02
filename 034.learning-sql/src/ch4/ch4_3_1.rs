#![allow(dead_code)]
use crate::entity::{customer, prelude::*, rental};
use anyhow::Result;
use sea_orm::{
    sea_query::{Expr, Func, Iden},
    DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter, QuerySelect,
};

#[derive(Debug, FromQueryResult)]
struct EqualityConditionsRes {
    email: String,
}

struct Date;

impl Iden for Date {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "date").unwrap();
    }
}

pub async fn equality_conditions(db: &DatabaseConnection) -> Result<()> {
    let customer = Customer::find()
        .select_only()
        .column(customer::Column::Email)
        .inner_join(Rental)
        .filter(
            Func::cust(Date)
                .args(vec![Expr::col(rental::Column::RentalDate)])
                .equals(Expr::val("2005-06-14")),
        )
        // .filter(rental::Column::RentalDate.between("2005-06-14", "2005-06-15"))
        .into_model::<EqualityConditionsRes>()
        .all(db)
        .await?;

    println!("{:#?}", customer);
    Ok(())
}
