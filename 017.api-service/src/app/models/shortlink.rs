use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlQueryResult;
use sqlx::{Error, FromRow, MySql, Pool};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ShortLink {
    pub id: u32,
    pub url: String,
}

pub async fn create_shortlink(pool: &Pool<MySql>, url: &str) -> Result<MySqlQueryResult, Error> {
    sqlx::query(
        r#"
            INSERT INTO short_links (`url`)
            VALUES(?)
        "#,
    )
    .bind(url)
    .execute(pool)
    .await
}

pub async fn delete_shortlink(pool: &Pool<MySql>, id: u64) -> Result<MySqlQueryResult, Error> {
    sqlx::query(
        r#"
            DELETE FROM short_links
            WHERE id = ?
        "#,
    )
    .bind(id)
    .execute(pool)
    .await
}

pub async fn get_shortlink(pool: &Pool<MySql>, id: i32) -> Result<ShortLink, Error> {
    sqlx::query_as::<_, ShortLink>(
        r#"
            SELECT * FROM short_links
            WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}
