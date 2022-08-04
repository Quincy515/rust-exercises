mod ch3;
mod ch4;
mod ch5;

pub mod ch6;
pub mod ch8;
pub mod ch9;
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

    ch9::ch9_3_1::all_operator(&db).await?;
    // ch9::ch9_3_1::multiple_row_single_column_subqueries(&db).await?;
    // ch9::ch9_3::noncorrelated_subqueries(&db).await?;
    // ch9::ch9_1::what_is_a_subquery(&db).await?;
    // ch8::ch8_4::group_filter_conditions(&db).await?;
    // ch8::ch8_3_3::grouping_via_expressions(&db).await?;
    // ch8::ch8_3_2::multicolumn_grouping(&db).await?;
    // ch8::ch8_3_1::single_column_grouping(&db).await?;
    // ch8::ch8_2::using_expressions(&db).await?;
    // ch8::ch8_2::aggregate_functions_customer(&db).await?;
    // ch8::ch8_2::aggregate_functions(&db).await?;
    // ch8::ch8_1::group_by_order_by_having(&db).await?;
    // ch8::ch8_1_1::group_by_order_by(&db).await?;
    // ch8::ch8_1_1::group_by(&db).await?;
    // ch6::ch6_3_1::union_all(&db).await?;
    // ch5::ch5_3::self_join(&db).await?;
    // ch5::ch5_2::same_table_twice_referring(&db).await?;
    // ch5::ch5_2::same_table_twice(&db).await?;
    // ch5::ch5_2::joining_three_or_more_tables(&db).await?;
    // ch5::ch5_1::inner_join_sql92(&db).await?;
    // ch5::ch5_1::inner_join(&db).await?;
    // ch4::ch4_4_1::never_equal_null_or(&db).await?;
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
