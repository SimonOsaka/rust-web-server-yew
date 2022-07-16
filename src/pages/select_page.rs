use crate::components::select::{Select, SelectColors, SelectSizes};
use yew::{function_component, html, Callback};

#[function_component(SelectPage)]
pub fn select_page() -> Html {
    let callback = Callback::from(|str: String| {
        gloo_console::log!(str);
    });

    html! {
        <>
            <Select {callback}>
                <option>{"Rust"}</option>
                <option>{"Go"}</option>
                <option>{"C"}</option>
                <option>{"C++"}</option>
            </Select>

            <Select>
                <option>{"Go"}</option>
                <option>{"C"}</option>
                <option selected={true}>{"Rust"}</option>
                <option>{"C++"}</option>
            </Select>

            <Select disable={true}>
                <option>{"Go"}</option>
                <option>{"C"}</option>
                <option>{"Rust"}</option>
                <option>{"C++"}</option>
            </Select>

            <Select round={true}>
                <option>{"Go"}</option>
                <option>{"C"}</option>
                <option>{"Rust"}</option>
                <option>{"C++"}</option>
            </Select>

            <Select color={SelectColors::Primary}>
                <option>{"Go"}</option>
                <option>{"C"}</option>
                <option>{"Rust"}</option>
                <option>{"C++"}</option>
            </Select>

            <Select size={SelectSizes::Large}>
                <option>{"Go"}</option>
                <option>{"C"}</option>
                <option>{"Rust"}</option>
                <option>{"C++"}</option>
            </Select>

            <Select icon={"fas fa-envelope"} loading={true}>
                <option>{"Go"}</option>
                <option>{"C"}</option>
                <option>{"Rust"}</option>
                <option>{"C++"}</option>
            </Select>
        </>
    }
}
