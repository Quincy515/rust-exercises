use leptos::*;

#[component]
pub fn StaticView() -> impl IntoView {
    let values = vec![0, 1, 2];

    view! {
        <p>{values.clone()}</p>
        <ul>
            {values.into_iter()
                .map(|n| view!{<li>{n}</li>})
                .collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
pub fn StaticList(length: usize) -> impl IntoView {
    let counters = (1..=length).map(|idx| create_signal(idx));
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button
                        class="btn btn-square btn-outline"
                        on:click=move|_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <ul>{counter_buttons}</ul>
    }
}
#[component]
pub fn DynamicList(initial_length: usize) -> impl IntoView {
    let mut next_counter_id = initial_length;
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();
    let (counters, set_counters) = create_signal(initial_counters);
    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        next_counter_id += 1;
    };
    view! {
        <div>
            <button on:click = add_counter>
                "Add counter"
            </button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    view = move|(id, (count, set_count))|{
                        view!{
                            <li>
                                <button
                                    class="btn btn-square btn-outline"
                                    on:click=move|_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    class="btn btn-circle btn-outline"
                                    on:click=move|_|{
                                        set_counters.update(|counters|{
                                            counters.retain(|(idx, _)| *idx != id);
                                        })
                                    }
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
