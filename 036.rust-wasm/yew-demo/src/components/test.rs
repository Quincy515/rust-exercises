use crate::helper::js;
use yew::prelude::*;

pub enum Msg {
    TestClick,
}

// 组件对象
pub struct TestComp {
    name: String,
}

impl Component for TestComp {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: String::from("test"),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TestClick => {
                js::alert("按钮点击");
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
            <button onclick={link.callback(|_|Msg::TestClick)}>{"按钮"}</button>
            <h1>{&self.name}</h1>
            </div>
        }
    }
}
