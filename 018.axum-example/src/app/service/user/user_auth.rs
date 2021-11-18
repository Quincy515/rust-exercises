use crate::app::model::user::user::User;
use crate::config::databases::Pool;
use diesel::insert_into;

use crate::app::model::user::request::user::UserRegister;
use crate::schema::users::dsl::*;
use diesel::prelude::*;

pub struct UserService;

impl UserService {
    pub fn find(pool: &Pool, uid: i32) -> QueryResult<User> {
        let conn = pool.get().unwrap();
        users.filter(id.eq(uid)).get_result::<User>(&conn)
    }

    pub fn find_username(pool: &Pool, name: &str) -> QueryResult<User> {
        let conn = pool.get().unwrap();
        users.filter(username.eq(name)).get_result::<User>(&conn)
    }

    pub fn create(pool: &Pool, input: UserRegister) -> QueryResult<User> {
        let conn = pool.get().unwrap();
        insert_into(users).values(input).execute(&conn)?;
        let last_id = Self::last_id(pool)?;
        Self::find(pool, last_id)
    }

    pub fn last_id(pool: &Pool) -> QueryResult<i32> {
        let conn = pool.get().unwrap();
        users.select(id).order(id.desc()).first(&conn)
    }
}
