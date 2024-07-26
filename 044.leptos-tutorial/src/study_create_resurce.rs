use gloo_timers::future::TimeoutFuture;
use leptos::*;
use leptos::{component, view, IntoView};

#[component]
pub fn StudyCreateResource() -> impl IntoView {
    // this count is our synchronous, local state
    let (count, set_count) = create_signal(0);
    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        // the first is the "source signal"
        count,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move {
            logging::log!("loading data from API");
            load_data(value).await
        },
    );
    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on anything: we just load it once

    let once = create_resource(|| (), |_| async move { load_data(2).await });

    // we can access the resource values with .read()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolve
    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            .unwrap_or_else(|| "Loading...".into())
    };
    // the resource's loading() method gives us a
    // signal to indicate whether it's currently loading

    let loading = async_data.loading();
    let is_loading = move || {
        if loading() {
            view! { <span class="loading loading-infinity text-primary"></span> }.into_view()
        } else {
            view! { <span>"Idle."</span> }.into_view()
        }
    };

    view! {
        <button
            class="btn btn-neutral"
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >

            "Click me count:"
            {count}
        </button>
        <p>
            <code>"async_value"</code>
            ": "
            {async_result}
            <br/>
            {is_loading}
        </p>

        <p>
            <code>"Once Data"</code>
            ": "
            {move || match once.get() {
                None => {
                    view! { <span class="loading loading-infinity text-primary"></span> }
                        .into_view()
                }
                Some(data) => view! { <code>{data}</code> }.into_view(),
            }}

        </p>
    }
}

async fn load_data(value: i32) -> i32 {
    TimeoutFuture::new(4_000).await;
    value * 10
}
