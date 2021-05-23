use async_graphql::{Error, ErrorExtensions};
use futures::stream::StreamExt;
use mongodb::Database;

use crate::users::models::User;
use crate::util::constant::GqlResult;

pub async fn all_users(db: Database) -> GqlResult<Vec<User>> {
    let coll = db.collection("user");
    let mut users: Vec<User> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(None, None).await.unwrap();

    // Iterator over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let user = bson::from_bson(bson::Bson::Document(document)).unwrap();
                users.push(user);
            }
            Err(error) => Err(Error::new("6-all-users")
                .extend_with(|_, e| e.set("details", format!("Error to find doc:{}", error))))
            .unwrap(),
        }
    }
    if users.len() > 0 {
        Ok(users)
    } else {
        Err(Error::new("6-all-users").extend_with(|_, e| e.set("details", "No records")))
    }
}
