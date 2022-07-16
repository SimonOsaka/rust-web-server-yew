use crate::components::checkbox::Checkbox;
use yew::{function_component, html, Callback};

#[function_component(CheckboxPage)]
pub fn checkbox_page() -> Html {
    let callback = Callback::from(|checked: bool| {
        gloo_console::log!(checked);
    });

    html! {
        <>
            <div class="block">
                <Checkbox {callback}>{"Remember me"}</Checkbox>
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
