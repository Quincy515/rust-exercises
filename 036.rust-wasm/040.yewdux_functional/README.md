[Function Yewdux](#function-yewdux)
[Install Yewdux](#install-yewdux)
[Create store](#create-store)
[Create Form](#create-form)
[Store form data into store](#store-form-data-into-store)
[Display form data](#display-form-data)
[Login Button](#login-button)

# Function Yewdux

https://www.youtube.com/watch?v=8C_pPdrvvWg&list=PLrmY5pVcnuE_R5qJ0o30eGw77bWmnrUtL&index=40

## Install Yewdux

```shell
cargo add yew --features yew/csr web-sys --features web-sys/HtmlInputElement yewdux gloo wasm-bindgen
```

## Create store

00:58

```rust
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Store)]
pub struct AuthStore {
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_authenticated: bool,
}
```

## Create Form

02:55

```rust
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    html! {
        <form>
            <h2>{"Login"}</h2>
            <div>
                <div>
                    <label for="username">{"Username"}</label>
                </div>
                <div>
                    <input type="text" id="username" placeholder="Username" />
                </div>
                <div>
                    <label for="password">{"Password"}</label>
                </div>
                <div>
                    <input type="password" id="password" placeholder="password" />
                </div>
            </div>
            <div>
                <button type="submit">{"Login"}</button>
            </div>
        </form>
    }
}
```

## Store form data into store

```rust
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::auth_store::AuthStore;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let (_, auth_dispatch) = use_store::<AuthStore>();
    let onchange_username = {
        let dispatch = auth_dispatch.clone();

        Callback::from(move |e: Event| {
            let username: String = e.target_unchecked_into::<HtmlInputElement>().value();
            let username = if username.is_empty() {
                None
            } else {
                Some(username)
            };
            dispatch.reduce_mut(|store| store.username = username);
        })
    };
    let onchange_password = auth_dispatch.reduce_mut_callback_with(|store, event: Event| {
        let password: String = event.target_unchecked_into::<HtmlInputElement>().value();
        store.password = if password.is_empty() {
            None
        } else {
            Some(password)
        };
    });
    html! {
        <form>
            <h2>{"Login"}</h2>
            <div>
                <div>
                    <label for="username">{"Username"}</label>
                </div>
                <div>
                    <input type="text" id="username" placeholder="Username" onchange={onchange_username}/>
                </div>
                <div>
                    <label for="password">{"Password"}</label>
                </div>
                <div>
                    <input type="password" id="password" placeholder="password" onchange={onchange_password}/>
                </div>
            </div>
            <div>
                <button type="submit">{"Login"}</button>
            </div>
        </form>
    }
}
```

## Display form data

14:45

```rust
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::auth_store::AuthStore;

#[function_component(DisplayAuth)]
pub fn display_auth() -> Html {
    let (store, _) = use_store::<AuthStore>();
    let username = format!(
        "Username: {}",
        store.username.as_deref().unwrap_or_default()
    );
    let password = format!(
        "Password: {}",
        store.password.as_deref().unwrap_or_default()
    );
    let is_authenticated = format!("Is Authenticated: {}", store.is_authenticated);
    html! {
        <div>
            <h2>{"Auth Data"}</h2>
            <div>{username}</div>
            <div>{password}</div>
            <div>{is_authenticated}</div>
        </div>
    }
}
```

## Login Button

18:22
```rust
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::auth_store::AuthStore;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let (_, auth_dispatch) = use_store::<AuthStore>();
    let onchange_username = {
        let dispatch = auth_dispatch.clone();

        Callback::from(move |e: Event| {
            let username: String = e.target_unchecked_into::<HtmlInputElement>().value();
            let username = if username.is_empty() {
                None
            } else {
                Some(username)
            };
            dispatch.reduce_mut(|store| store.username = username);
        })
    };
    let onchange_password = auth_dispatch.reduce_mut_callback_with(|store, event: Event| {
        let password: String = event.target_unchecked_into::<HtmlInputElement>().value();
        store.password = if password.is_empty() {
            None
        } else {
            Some(password)
        };
    });
    let onsubmit = auth_dispatch.reduce_mut_callback_with(|store, event: SubmitEvent| {
        event.prevent_default();
        store.is_authenticated = store.username.is_some() && store.password.is_some();
    });
    html! {
        <form {onsubmit}>
            <h2>{"Login"}</h2>
            <div>
                <div>
                    <label for="username">{"Username"}</label>
                </div>
                <div>
                    <input type="text" id="username" placeholder="Username" onchange={onchange_username}/>
                </div>
                <div>
                    <label for="password">{"Password"}</label>
                </div>
                <div>
                    <input type="password" id="password" placeholder="password" onchange={onchange_password}/>
                </div>
            </div>
            <div>
                <button type="submit">{"Login"}</button>
            </div>
        </form>
    }
}
```



