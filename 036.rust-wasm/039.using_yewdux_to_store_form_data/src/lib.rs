mod auth_form;
mod display_auth;
mod stores;

use yew::prelude::*;

use crate::{auth_form::AuthForm, display_auth::DisplayAuth};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h1>{"App"}</h1>
            <AuthForm />
            <DisplayAuth />
        </div>
    }
}
