use leptos::*;
use leptos::{component, create_rw_signal, create_signal, provide_context, view, IntoView};

#[component]
pub fn Effect() -> impl IntoView {
    let log = create_rw_signal::<Vec<String>>(vec![]);
    let logged = move || log().join("\n");
    provide_context(log);

    view! {
        <CreateAnEffect/>
        <pre>{logged}</pre>
    }
}

#[component]
pub fn CreateAnEffect() -> impl IntoView {
    let (first, set_first) = create_signal(String::new());
    let (last, set_last) = create_signal(String::new());
    let (use_last, set_use_last) = create_signal(true);

    create_effect(move |_| {
        log(if use_last() {
            format!("{} {}", first(), last())
        } else {
            first()
        })
    });

    view! {
        <h1>
            <code>"Create Effect"</code>
            " Version"
        </h1>
        <form>
            <label>
                "First Name"
                <input
                    type="text"
                    name="first"
                    prop:value=first
                    on:change=move |ev| set_first(event_target_value(&ev))
                />
            </label>
            <label>
                "Last Name"
                <input
                    type="text"
                    name="last"
                    prop:value=last
                    on:change=move |ev| set_last(event_target_value(&ev))
                />
            </label>
            <label>
                "Show Last Name"
                <input
                    type="checkbox"
                    name="use_last"
                    prop:checked=use_last
                    on:change=move |ev| set_use_last(event_target_checked(&ev))
                />
            </label>
        </form>
    };
}

fn log(msg: impl std::fmt::Display) {
    let log = use_context::<RwSignal<Vec<String>>>().unwrap();
    log.update(|log| log.push(msg.to_string()));
}
