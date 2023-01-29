mod display_count;
mod increment_count;
mod stores;

use yew::prelude::*;

use crate::{display_count::DisplayCount, increment_count::IncrementCount};

#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <h1>{"App"}</h1>
            <DisplayCount />
            <IncrementCount />
        </>
    }
}
