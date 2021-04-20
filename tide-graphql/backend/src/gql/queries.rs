use async_graphql::Context;

use crate::dbs::mongo::DataSource;
use crate::users::{self, models::User};
use crate::util::constant::GqlResult;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // Get all Users
    async fn all_users(&self, ctx: &Context<'_>) -> GqlResult<Vec<User>> {
        let db = ctx.data_unchecked::<DataSource>().db_custer.clone();
        users::services::all_users(db).await
    }
}
