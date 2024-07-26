use leptos::html::Input;
use leptos::*;
use leptos::{component, create_rw_signal, create_signal, provide_context, view, IntoView};

#[component]
pub fn CreateEffect() -> impl IntoView {
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

#[component]
pub fn ManualVersion() -> impl IntoView {
    let first = create_node_ref::<Input>();
    let last = create_node_ref::<Input>();
    let use_last = create_node_ref::<Input>();

    let mut prev_name = String::new();
    let on_change = move |_| {
        log(" listener");
        let first = first.get().unwrap();
        let last = last.get().unwrap();
        let use_last = use_last.get().unwrap();
        let this_one = if use_last.checked() {
            format!("{} {}", first.value(), last.value())
        } else {
            first.value()
        };

        if this_one != prev_name {
            log(&this_one);
            prev_name = this_one;
        }
    };
    view! {
        <h1>"Manual Version"</h1>
        <form on:change=on_change>
            <label>"First Name" <input type="text" name="fist" node_ref=first/></label>
            <label>"Last Name" <input type="text" name="last" node_ref=last/></label>
            <label>
                "Show Last Name" <input type="checkbox" name="use_last" checked node_ref=use_last/>
            </label>
        </form>
    }
}

#[component]
pub fn EffectVsDerivedSignal() -> impl IntoView {
    let (my_value, set_my_value) = create_signal(String::new());
    // Don't do this.
    /*let (my_optional_value, set_optional_my_value) = create_signal(cx, Option::<String>::None);

    create_effect(cx, move |_| {
        if !my_value.get().is_empty() {
            set_optional_my_value(Some(my_value.get()));
        } else {
            set_optional_my_value(None);
        }
    });*/

    // Do this
    let my_optional_value =
        move || (!my_value.with(String::is_empty)).then(|| Some(my_value.get()));

    view! {
        <input prop:value=my_value on:input=move |ev| set_my_value(event_target_value(&ev))/>

        <p>
            <code>"my_optional_value"</code>
            " is "
            <code>
                <Show when=move || my_optional_value().is_some() fallback=|| view! { "None" }>
                    "Some("
                    "
                    {my_optional_value().unwrap()}
                    "
                    ")"
                </Show>
            </code>
        </p>
    }
}
