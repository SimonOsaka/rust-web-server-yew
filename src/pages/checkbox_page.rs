use crate::components::checkbox::Checkbox;
use yew::{function_component, html, use_state, Callback};

#[function_component(CheckboxPage)]
pub fn checkbox_page() -> Html {
    let state = use_state(|| false);
    let callback = {
        let state = state.clone();
        Callback::from(move |checked: bool| {
            gloo_console::log!(checked);
            state.set(checked);
        })
    };

    html! {
        <>
            <div class="block">
                <Checkbox {callback} check={*state}>{"Remember me"}</Checkbox>
            </div>

            <div class="block">
                <Checkbox check=true>{"Remember me"}</Checkbox>
            </div>

            <div class="block">
                <Checkbox disable=true>{"Remember me"}</Checkbox>
            </div>

            <div class="block">
                <Checkbox>{"I agree to the"} <a href="#">{"terms and conditions"}</a></Checkbox>
            </div>
        </>
    }
}
