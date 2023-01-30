use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let show = use_state(|| true);

    let show_clone = show.clone();
    gloo::timers::callback::Timeout::new(5000, move || {
        show_clone.set(false);
    })
    .forget();

    html! {
        <div>
            <h1>{"APP"}</h1>
            if *show {
                <p>{"Hello, world!"}</p>
            }
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
