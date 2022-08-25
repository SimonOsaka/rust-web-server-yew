use rust_web_server_yew::{
    components::radio::{Radio, RadioGroup},
    log,
};
use yew::{function_component, html, use_state, Callback};

#[function_component(RadioPage)]
pub fn radio_page() -> Html {
    let state = use_state(|| "no".to_string());
    let callback = {
        let state = state.clone();
        Callback::from(move |value: String| {
            log!(value.clone());
            state.set(value);
        })
    };

    html! {
        <>
            <div class="block">
                <Radio label={"Yes"} value={"yes"} />
            </div>

            <div class="block">
                <Radio label={"No"} value={"no"} check={true} />
            </div>

            <div class="block">
                <RadioGroup {callback} value={(*state).clone()}>
                    <Radio label={"Yes"} value={"yes"} />
                    <Radio label={"No"} value={"no"} />
                </RadioGroup>
            </div>
        </>
    }
}
