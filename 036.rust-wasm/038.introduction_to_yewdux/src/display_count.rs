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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Store(_store) => true,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let count = self.dispatch.get().count;
        html! {
            <div>
                <h1>{ "Count" }</h1>
                <div>{count}</div>
            </div>
        }
    }
}
