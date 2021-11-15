use diesel::result::QueryResult;
use diesel::prelude::*;
use super::models::*;
use super::schema::*;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn load_all(c: &SqliteConnection) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(100).load::<Rustacean>(c)
    }

    pub fn find(c: &SqliteConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    pub fn create(c: &SqliteConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .execute(c)?;

        let last_id = Self::last_id(c)?;

        Self::find(c, last_id)
    }

    pub fn save(c: &SqliteConnection, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(rustacean.id))
            .set((
                rustaceans::name.eq(rustacean.name.to_owned()),
                rustaceans::email.eq(rustacean.email.to_owned()),
            ))
            .execute(c)?;

        Self::find(c, rustacean.id)
    }

    pub fn delete(c: &SqliteConnection, id: i32) -> QueryResult<usize>  {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }

    fn last_id(c: &SqliteConnection) -> QueryResult<i32> {
        rustaceans::table
            .select(rustaceans::id)
            .order(rustaceans::id.desc())
            .first(c)
    }
}