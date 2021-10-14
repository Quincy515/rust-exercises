use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

pub async fn do_connect() -> Pool<MySql> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root1234@localhost/shorten_db")
        .await;
    pool.unwrap()
}
