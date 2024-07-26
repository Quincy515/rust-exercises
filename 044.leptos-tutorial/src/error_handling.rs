use leptos::*;
use leptos::{
    component, create_signal, event_target_value, view, CollectView, ErrorBoundary, IntoView,
    SignalGet,
};

#[component]
pub fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <label>
            "Type a number (or not!)" <input type="number" on:input=on_input/>
            <p>"You entered" <strong>{value}</strong></p>
        </label>
    }
}

#[component]
pub fn NumericInputErrorBoundary() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or not!)"
            <input type="number" class="input w-full max-w-xs" on:input=on_input/>
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="alert alert-error">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="stroke-current shrink-0 h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                            ></path>
                        </svg>
                        <span>"Not a number! Errors:"</span>
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            }}

                        </ul>
                    </div>
                }
            }>

                <p>"You entered" <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}
