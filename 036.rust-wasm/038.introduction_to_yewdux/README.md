- [Setting up Yewdux](#setting-up-yewdux)
  - [Install Yewdux](#install-yewdux)
  - [Create counter store with default value](#create-counter-store-with-default-value)
  - [Manual display counter](#manual-display-counter)
  - [Create button to increment count](#create-button-to-increment-count)


# Setting up Yewdux
https://www.youtube.com/watch?v=yBGsgrg0SKA&list=PLrmY5pVcnuE_R5qJ0o30eGw77bWmnrUtL&index=38

00:24

当我们使用多个组件，每个组件都需要来回共享数据

或者一个组件只显示另一个组件设置的数据

或者只想在数据刷新时更新组件



Centralized Store

## Install Yewdux



```toml

[dependencies]

gloo = "0.8"

yew = {version = "0.20", fetures = ["csr"]}

yewdux = "0.9"

```



## Create counter store with default value



```rust

use yewdux::prelude::*;



#[derive(Default, Clone, PartialEq, Store)]

pub struct CounterStore {

    pub count: u32,

}

```

## Manual display counter

https://intendednull.github.io/yewdux/reading.html



```rust

use std::rc::Rc;



use yew::prelude::*;

use yewdux::prelude::*;



use crate::stores::counter_store::CounterStore;



pub enum Msg {

    Store(Rc<CounterStore>),

}

pub struct DisplayCount {

    dispatch: Dispatch<CounterStore>,

}



impl Component for DisplayCount {

    type Message = Msg;

    type Properties = ();



    fn create(ctx: &Context<Self>) -> Self {

        let callback = ctx.link().callback(Msg::Store);

        let dispatch = Dispatch::<CounterStore>::subscribe(callback);

        Self { dispatch }

    }



    fn view(&self, _ctx: &Context<Self>) -> Html {

        let count = self.dispatch.get().count;

        html! {

            <div>

                <h1>{ "Counter" }</h1>

                <div>{count}</div>

            </div>

        }

    }

}

```



## Create button to increment count



```rust

use std::rc::Rc;



use yew::prelude::*;

use yewdux::prelude::*;



use crate::stores::counter_store::CounterStore;



pub enum Msg {

    Store(Rc<CounterStore>),

    ButtonClicked,

}

pub struct IncrementCount {

    pub dispatch: Dispatch<CounterStore>,

}



impl Component for IncrementCount {

    type Message = Msg;

    type Properties = ();



    fn create(ctx: &Context<Self>) -> Self {

        let on_change = ctx.link().callback(Msg::Store);

        let dispatch = Dispatch::<CounterStore>::subscribe(on_change);

        Self { dispatch }

    }



    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {

            Msg::Store(_store) => false,

            Msg::ButtonClicked => {

                // self.dispatch.reduce(|store| {

                //     CounterStore {

                //         count: store.count + 1,

                //     }

                //     .into()

                // });

                self.dispatch.reduce_mut(|store| store.count += 1);

                false

            }

        }

    }



    fn view(&self, ctx: &Context<Self>) -> Html {

        let onclick = ctx.link().callback(|_| Msg::ButtonClicked);

        html! {

            <button onclick={onclick}>{"Increment Count"}</button>

        }

    }

}

```



















