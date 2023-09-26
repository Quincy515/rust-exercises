use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="btn"
                class:red=move||count()%2==1
                on:click=move |_| set_count.update(|n| *n += 1)
            >
                "Click me: "
                {move || count()}
            </button>
        </div>
    }
}
