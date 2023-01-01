mod components;
use components::test::TestComp;

fn main() {
    // yew::start_app::<TestComp>();
    yew::Renderer::<TestComp>::new().render();
}
