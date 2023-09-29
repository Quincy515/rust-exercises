use leptos::{
    component, create_node_ref, create_signal, ev::SubmitEvent, event_target_value, html::Input,
    view, IntoView, NodeRef,
};

#[component]
pub fn ControlledInputs() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <div class="form-control w-full max-w-xs">
            <label class="label">
                <span class="label-text">"Name is : "</span>
                <span class="label-text-alt">{name}</span>
            </label>
            <input
                type="text"
                placeholder="Type here"
                class="input input-primary w-full max-w-xs"
                on:input=move |ev| { set_name(event_target_value(&ev)) }

                prop:value=name
            />
        </div>
    }
}

#[component]
pub fn UncontrolledInputs() -> impl IntoView {
    let (name, set_name) = create_signal("UncontrolledInputs".to_string());
    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();
        set_name(value);
    };
    view! {
        <form on:submit=on_submit>
            <div class="join">
                <input
                    type="text"
                    value=name
                    node_ref=input_element
                    class="input input-primary w-full max-w-xs join-item"
                />
                <input class="btn join-item ml-1" type="submit" value="Submit"/>
            </div>
        </form>
        <p>"Name is: " {name}</p>
    }
}
