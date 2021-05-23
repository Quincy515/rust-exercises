mod dbs;
mod gql;
mod users;
mod util;

use crate::gql::{build_schema, graphiql, graphql};
use crate::util::constant::CFG;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
  // tide logger
  tide::log::start();

  // initial Tide application state
  let schema = build_schema().await;
  let app_state = State { schema: schema };
  let mut app = tide::with_state(app_state);

  // route confige
  app.at(CFG.get("GRAPHQL_PATH").unwrap()).post(graphql);
  app.at(CFG.get("GRAPHIQL_PATH").unwrap()).get(graphiql);

  app
    .listen(format!(
      "{}:{}",
      CFG.get("ADDRESS").unwrap(),
      CFG.get("PORT").unwrap()
    ))
    .await?;

  Ok(())
}

// Tide application scope state
#[derive(Clone)]
pub struct State {
  pub schema: async_graphql::Schema<
    gql::queries::QueryRoot,
    async_graphql::EmptyMutation,
    async_graphql::EmptySubscription,
  >,
}
