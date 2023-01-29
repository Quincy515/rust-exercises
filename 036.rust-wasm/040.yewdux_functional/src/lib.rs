mod display_form;
mod login_form;
mod store;

use yew::prelude::*;

use crate::{display_form::DisplayAuth, login_form::LoginForm};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h1>{ "App" }</h1>
            <LoginForm />
            <DisplayAuth />
        </div>
    }
}
