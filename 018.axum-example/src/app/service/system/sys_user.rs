use crate::schema::users::dsl::*;
use crate::util::encryption;
use crate::{
    app::model::user::{request::user::UserLogin, user::User},
    config::databases::Pool,
    error::{Result, WebError},
};
use diesel::prelude::*;

pub struct UserService;

impl UserService {
    pub async fn login(input: UserLogin, pool: &Pool) -> Result<User> {
        let conn = pool.get().unwrap();
        let user = users
            .filter(username.eq(input.username))
            .get_result::<User>(&conn)?;
        if encryption::verify_password(input.password.unwrap(), user.password.to_owned().unwrap())
            .await?
        {
            Ok(user)
        } else {
            Err(WebError::WrongCredentials)
        }
    }
}
