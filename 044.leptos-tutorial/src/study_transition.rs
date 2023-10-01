use gloo_timers::future::TimeoutFuture;
use leptos::*;

/// <Transition/>行为与<Suspense/>完全相同，
/// 但不是每次都回退，而是第一次只显示回退。
/// 在所有后续加载中，它继续显示旧数据，直到新数据准备就绪。
/// 这可以非常方便地防止闪烁效果，并允许用户继续与您的应用程序交互。
#[component]
pub fn StudyTransition() -> impl IntoView {
    let (tab, set_tab) = create_signal(0);
    let user_data = create_resource(tab, |tab| async move { important_api_call(tab).await });
    view! {
        <div class="tabs tabs-boxed">
            <a class="tab" class=("tab-active", move || tab() == 0) on:click=move |_| set_tab(0)>
                Tab 1
            </a>
            <a class="tab" class=("tab-active", move || tab() == 1) on:click=move |_| set_tab(1)>
                Tab 2
            </a>
            <a class="tab" class=("tab-active", move || tab() == 2) on:click=move |_| set_tab(2)>
                Tab 3
            </a>
            {move || {
                if user_data.loading().get() {
                    view! { <span class="loading loading-ring text-primary"></span> }
                } else {
                    view! { <span></span> }
                }
            }}
        </div>
        <Transition fallback=move || {
            view! { <span class="loading loading-ring text-primary"></span> }
        }>
            <p>{move || user_data.get()}</p>
        </Transition>
    }
}

async fn important_api_call(id: usize) -> String {
    TimeoutFuture::new(1_000).await;
    match id {
        0 => "Alice",
        1 => "Bob",
        2 => "Carol",
        _ => "User not found",
    }
    .to_string()
}
