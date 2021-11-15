use super::schema::rustaceans;

#[derive(serde::Serialize, serde::Deserialize, Queryable, AsChangeset)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Insertable, serde::Deserialize)]
#[table_name="rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}