use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Store)]
pub struct AuthStore {
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_authenticated: bool,
}
