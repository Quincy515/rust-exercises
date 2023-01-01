use yew::prelude::*;

pub enum Msg {}

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
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <h1>{&self.name}</h1>
            </div>
        }
    }
}
