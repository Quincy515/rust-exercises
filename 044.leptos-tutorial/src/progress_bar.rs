use leptos::{view, component, IntoView, Signal};

#[component]
pub fn ProgressBar(
    #[prop(default=100)]
    max:u16,
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view!{
        <progress
            class="progress progress-primary w-56"
            max=max
            value=progress
        />
    }
}
