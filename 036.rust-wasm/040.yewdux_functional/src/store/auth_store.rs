use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Store, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct AuthStore {
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_authenticated: bool,
}
