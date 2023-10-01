use leptos::*;

use crate::component_children::PassingChildrenToComponent;
use crate::create_effect::CreateAnEffect;
use crate::error_handling::{NumericInput, NumericInputErrorBoundary};
use crate::forms::{ControlledInputs, UncontrolledInputs};
use crate::iteration::{DynamicList, StaticList, StaticView};
use crate::parent_child_communication::Communicate;
use crate::progress_bar::ProgressBar;
use crate::study_create_resurce::StudyCreateResource;
use crate::study_suspense::StudySuspense;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    let html = "<p>This HTML will be injected.</p>";

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">
                "Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."
            </p>
            <button
                class=("btn", move || count() % 2 == 1)
                on:click=move |_| set_count.update(|n| *n += 1)
            >
                "Click me: "
                {move || count()}
            </button>
            <br/>
            <progress max="50" value=double_count></progress>
            <p>"Double Count: " {double_count}</p>
            <br/>
            <div inner_html=html></div>
            <br/>
            <ProgressBar progress=count/>
            <br/>
            <ProgressBar progress=Signal::derive(double_count)/>
            <br/>
            <StaticView/>
            <br/>
            <StaticList length=4/>
            <br/>
            <DynamicList initial_length=4/>
            <br/>
            <ControlledInputs/>
            <br/>
            <UncontrolledInputs/>
            <br/>
            <NumericInput/>
            <br/>
            <NumericInputErrorBoundary/>
            <br/>
            <Communicate/>
            <br/>
            <PassingChildrenToComponent/>
            <br/>
            <CreateAnEffect/>
            <br/>
            <StudyCreateResource/>
            <br/>
            <StudySuspense/>
        </div>
    }
}
