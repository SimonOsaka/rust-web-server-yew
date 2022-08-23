use crate::components::checkbox::Checkbox;
use gloo::console;
use yew::{function_component, html, use_state, Callback};

#[function_component(CheckboxPage)]
pub fn checkbox_page() -> Html {
    let state = use_state(|| false);
    let callback = {
        Callback::from(move |(checked, _)| {
            console::log!(checked);
            state.set(checked);
        })
    };

    html! {
        <>
            <div class="block">
                <Checkbox {callback}>{"Remember me"}</Checkbox>
            </div>

            <div class="block">
                <Checkbox init_checked=true>{"Remember me"}</Checkbox>
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
