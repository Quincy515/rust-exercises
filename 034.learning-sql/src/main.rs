mod ch3;
mod ch4;

pub mod entity;

use anyhow::Result;
use sea_orm::{ConnectOptions, Database};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let mut opt = ConnectOptions::new("mysql://root:root1234@localhost:3306/sakila".to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));
    let db = Database::connect(opt).await?;

    ch4::ch4_4_1::never_equal_null_or(&db).await?;
    // ch4::ch4_4_1::never_equal_not_null(&db).await?;
    // ch4::ch4_4_1::never_equal_null(&db).await?;
    // ch4::ch4_3_4::matching_conditions_regexp(&db).await?;
    // ch4::ch4_3_4::matching_conditions_mul(&db).await?;
    // ch4::ch4_3_4::matching_conditions(&db).await?;
    // ch4::ch4_3_3::membership_conditions_not_in(&db).await?;
    // ch4::ch4_3_3::membership_conditions_in(&db).await?;
    // ch4::ch4_3_3::membership_conditions(&db).await?;
    // ch4::ch4_3_2::range_conditions(&db).await?;
    // ch4::ch4_3_1::equality_conditions(&db).await?;
    // ch3::ch3_7::get_customer_order_by(&db).await?;
    // ch3::ch3_6::get_customer_group_by(&db).await?;
    // ch3::ch3_5::get_rating_film_mul_where(&db).await?;
    // ch3::ch3_5::get_rating_film_where(&db).await?;
    // ch3::ch3_4::subquery_customer_table_link(&db).await?;
    // ch3::ch3_4::subquery_customer_list(&db).await?;
    // ch3::ch3_3::get_actor_id(&db).await?;
    // ch3::ch3_3::get_language_column_as(&db).await?;
    // ch3::ch3_3::get_language_column(&db).await?;
    // ch3::ch3_3::get_language_list(&db).await?;
    // ch3::ch3_1::get_category_list(&db).await?;

    Ok(())
}
