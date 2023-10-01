use gloo_timers::future::TimeoutFuture;
use leptos::*;

#[component]
pub fn StudySuspense() -> impl IntoView {
    view! {
        <First/>
        <TwoAsync/>
        <UseAwait/>
        <UseSuspense/>
    }
}

#[component]
fn First() -> impl IntoView {
    let (count, _) = create_signal("hello".to_string());
    let once = create_resource(
        count,
        |count| async move { important_api_call(count).await },
    );

    view! {
        <p>
            <code>"一个异步请求"</code>
        </p>
        {move || match once.get() {
            None => view! { <span class="loading loading-ring text-primary"></span> }.into_view(),
            Some(data) => view! { <span>{data}</span> }.into_view(),
        }}
    }
}

#[component]
fn TwoAsync() -> impl IntoView {
    let (count, _) = create_signal("hello".to_string());
    let (count2, _) = create_signal("world".to_string());
    let a = create_resource(
        count,
        |count| async move { important_api_call(count).await },
    );
    let b = create_resource(
        count2,
        |count| async move { important_api_call(count).await },
    );
    view! {
        <p>
            <code>"使用Suspense"</code>
        </p>
        <Suspense fallback=move || {
            view! { <span class="loading loading-ring text-primary"></span> }
        }>
            <p>
                <code>"A"</code>
                {move || { a.get().map(|a| view! { <span>{a}</span> }) }}
            </p>
            <p>
                <code>"B"</code>
                {move || { b.get().map(|b| view! { <span>{b}</span> }) }}
            </p>
        </Suspense>
    };
}

#[component]
fn UseAwait() -> impl IntoView {
    view! {
        <Await
            future=|| important_api_call("await".to_string())
            children=|data| {
                view! {
                    <p>
                        "<" {data.clone()}
                        "/> 本质上将 create_resource 与源参数|| ()与没有回退的<Suspense/>相结合。"
                    </p>
                }
            }
        />
    }
}

#[component]
fn UseSuspense() -> impl IntoView {
    let (name, set_name) = create_signal("Your Name".to_string());
    let async_data = create_resource(name, |name| async move { important_api_call(name).await });

    view! {
        <input
            type="text"
            class="input w-full max-w-xs"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }

            prop:value=name
        />
        <p>
            <code>"Name: "</code>
        </p>
        <Suspense fallback=move || {
            view! { <span class="loading loading-ring text-primary"></span> }
        }>
            <code>"Your shouting name is: " {move || async_data.get()}</code>
        </Suspense>
    }
}

async fn important_api_call(name: String) -> String {
    TimeoutFuture::new(4_000).await;
    name.to_ascii_uppercase()
}
