use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
  pub _id: ObjectId,
  pub email: String,
  pub username: String,
  pub cred: String,
}

#[async_graphql::Object]
impl User {
  // pub async fn id(&self) -> ObjectId {
  //   self._id.clone()
  // }

  pub async fn email(&self) -> &str {
    self.email.as_str()
  }

  pub async fn username(&self) -> &str {
    self.username.as_str()
  }
  #[graphql(name = "short_description")]
  pub async fn short_description(&self) -> String {
    "User".to_string()
  }
}
