- [Yewdux with forms](#yewdux-with-forms)
  - [Install Yewdux](#install-yewdux)
  - [Create the store](#create-the-store)
  - [Create Form](#create-form)
  - [Add form data to store when changed](#add-form-data-to-store-when-changed)
  - [Display from data](#display-from-data)

# Yewdux with forms

https://www.youtube.com/watch?v=2tUrJyt8U4A&list=PLrmY5pVcnuE_R5qJ0o30eGw77bWmnrUtL&index=39



## Install Yewdux

00:52


```shell

cargo add yew --features csr web-sys --features HtmlInputElement yewdux gloo wasm-bindgen

```



## Create the store

01:07


新建文件夹 `stores` 和 文件 `mod.rs`、`auth_store.rs`

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

04:27


 创建文件 `auth_form.rs`



```rust

 use std::rc::Rc;



use web_sys::HtmlInputElement;

use yew::prelude::*;

use yewdux::prelude::*;



use crate::stores::auth_store::AuthStore;



pub enum Msg {

    Store(Rc<AuthStore>),

    Username(String),

    Password(String),

    Login,

}

pub struct AuthForm {

    dispatch: Dispatch<AuthStore>,

}



impl Component for AuthForm {

    type Message = Msg;



    type Properties = ();



    fn create(ctx: &Context<Self>) -> Self {

        let dispatch = Dispatch::<AuthStore>::subscribe(ctx.link().callback(Msg::Store));

        Self { dispatch }

    }



    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {

            <form>

                <h2>{"Login"}</h2>

                <div>

                    <div>

                        <label for="username">{"Username"}</label>

                    </div>

                    <div>

                        <input type="text" id="username" placehold="username" />

                    </div>

                    <div>

                        <label for="password">{"Password"}</label>

                    </div>

                    <div>

                        <input type="password" id="password" placehold="password" />

                    </div>

                </div>

                <div>

                    <button>{"Login"}</button>

                </div>

            </form>

        }

    }

}



```



## Add form data to store when changed



```rust

use std::rc::Rc;



use web_sys::HtmlInputElement;

use yew::prelude::*;

use yewdux::prelude::*;



use crate::stores::auth_store::AuthStore;



pub enum Msg {

    Store(Rc<AuthStore>),

    Username(String),

    Password(String),

    Login,

}

pub struct AuthForm {

    dispatch: Dispatch<AuthStore>,

}



impl Component for AuthForm {

    type Message = Msg;



    type Properties = ();



    fn create(ctx: &Context<Self>) -> Self {

        let dispatch = Dispatch::<AuthStore>::subscribe(ctx.link().callback(Msg::Store));

        Self { dispatch }

    }



    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {

            Msg::Store(_store) => false,

            Msg::Username(username) => {

                self.dispatch

                    .reduce_mut(|store| store.username = Some(username));

                false

            }

            Msg::Password(password) => {

                self.dispatch

                    .reduce_mut(|store| store.password = Some(password));

                false

            }

            Msg::Login => {

                self.dispatch.reduce_mut(|store| {

                    store.is_authenticated = store.password.is_some() && store.username.is_some();

                });

                false

            }

        }

    }



    fn view(&self, ctx: &Context<Self>) -> Html {

        let onsubmit = ctx.link().callback(|event: SubmitEvent| {

            event.prevent_default();

            Msg::Login

        });

        let username_onchange = ctx.link().callback(|event: Event| {

            let target = event.target_unchecked_into::<HtmlInputElement>();

            let username = target.value();

            Msg::Username(username.to_string())

        });

        let password_onchange = ctx.link().callback(|event: Event| {

            let target = event.target_unchecked_into::<HtmlInputElement>();

            let password = target.value();

            Msg::Password(password.to_string())

        });

        html! {

            <form {onsubmit}>

                <h2>{"Login"}</h2>

                <div>

                    <div>

                        <label for="username">{"Username"}</label>

                    </div>

                    <div>

                        <input type="text" id="username" placehold="username" onchange={username_onchange}/>

                    </div>

                    <div>

                        <label for="password">{"Password"}</label>

                    </div>

                    <div>

                        <input type="password" id="password" placehold="password" onchange={password_onchange}/>

                    </div>

                </div>

                <div>

                    <button>{"Login"}</button>

                </div>

            </form>

        }

    }

}

```

## Display from data

22:43
新建文件 `display_auth.rs`

```rust

use std::rc::Rc;



use yew::prelude::*;

use yewdux::prelude::*;



use crate::stores::auth_store::AuthStore;



pub enum Msg {

    Store(Rc<AuthStore>),

}

pub struct DisplayAuth {

    dispatch: Dispatch<AuthStore>,

}



impl Component for DisplayAuth {

    type Message = Msg;

    type Properties = ();



    fn create(ctx: &Context<Self>) -> Self {

        let dispatch = Dispatch::<AuthStore>::subscribe(ctx.link().callback(Msg::Store));

        Self { dispatch }

    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {

            Msg::Store(_) => true,

        }

    }



    fn view(&self, _ctx: &Context<Self>) -> Html {

        let auth_store = self.dispatch.get();

        let username = format!(

            "Username: {}",

            auth_store.username.as_deref().unwrap_or_default()

        );

        let password = format!(

            "Password: {}",

            auth_store.password.as_deref().unwrap_or_default()

        );

        let is_authenticated = format!("Is Authenticated: {}", auth_store.is_authenticated);

        html! {

            <div>

                <h2>{"Auth Store"}</h2>

                <div>{username}</div>

                <div>{password}</div>

                <div>{is_authenticated}</div>

            </div>

        }

    }

}

```



