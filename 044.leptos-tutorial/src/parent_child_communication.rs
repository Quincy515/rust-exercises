use leptos::*;
use leptos::{
    component, create_signal, provide_context, use_context, view, IntoView, SignalUpdate,
    WriteSignal,
};

use leptos::ev::MouseEvent;

#[component]
pub fn Communicate() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);

    // 与此组件的所有子组件共享“set_toggled”
    provide_context(set_toggled);

    view! {
        <p>"Toggled? " {toggled}</p>
        <PassWriteSingal setter=set_toggled/>
        <UseCallback on_click=move |_| set_toggled.update(|value| *value = !*value)/>
        // 注意 on:click 而不是 on_click
        // 这与 HTML 元素事件监听器的语法相同
        <UserEventListener on:click=move |_| set_toggled.update(|value| *value = !*value)/>
        <ProvidingContext/>
    }
}

#[component]
fn PassWriteSingal(setter: WriteSignal<bool>) -> impl IntoView {
    view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle"</button> }
}

#[component]
fn UseCallback<F>(on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! { <input type="checkbox" class="toggle" on:click=on_click/> }
}

#[component]
fn UserEventListener() -> impl IntoView {
    view! { <input type="checkbox" class="toggle"/> }
}

#[component]
fn ProvidingContext() -> impl IntoView {
    view! {
        <header>
            <h1>"My Page"</h1>
        </header>
        <main>
            <Content/>
        </main>
    }
}

#[component]
fn Content() -> impl IntoView {
    view! {
        <div class="content">
            <ContextButton/>
        </div>
    }
}
#[component]
fn ContextButton() -> impl IntoView {
    // use_context 搜索上下文树，希望能够
    // 找到一个 `WriteSignal<bool>`
    // 在这种情况下，我 .expect() 因为我知道我提供了它
    let setter = use_context::<WriteSignal<bool>>().expect("to have found the setter provided ");
    view! {
        <input
            type="checkbox"
            class="toggle"
            on:click=move |_| setter.update(|value| *value = !*value)
        />
    }
}
