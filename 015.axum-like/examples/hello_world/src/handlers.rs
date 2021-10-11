use serde::Deserialize;
use tracing::info;
use axumlike::extract::{Query, TypedHeader};
use axumlike::response::IntoResponse;


// pub(crate) async fn handler() -> &'static str {
//     "<h1>Hello, World!</h1>"
// }
pub async fn handler(user_agent: Option<TypedHeader<headers::UserAgent>>) -> impl IntoResponse {
    let url = "localhost";
    if let Some(TypedHeader(user_agent)) = user_agent {
        tracing::info!(%url, user_agent = ?user_agent.as_str(),
        "Got a connection!");
    }

    let res = "<h1>Hello, World!</h1>".into_response();
    res
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    page: usize,
    per_page: usize,
}

pub(crate) async fn page_handler(pagination: Query<Pagination>) -> &'static str {
    let pagination: Pagination = pagination.0;

    info!(?pagination, "Got a connection!");

    "<h1>Hello, World!</h1>"
}