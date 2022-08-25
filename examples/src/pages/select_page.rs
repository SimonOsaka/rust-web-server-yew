use rust_web_server_yew::{
    components::{select::Select, Color, Size},
    log,
};
use yew::{function_component, html, Callback};

#[function_component(SelectPage)]
pub fn select_page() -> Html {
    let callback = Callback::from(|str: String| {
        log!(str);
    });

    html! {
        <>
            <div class="block">
                <Select {callback}>
                    <option>{"Rust"}</option>
                    <option>{"Go"}</option>
                    <option>{"C"}</option>
                    <option>{"C++"}</option>
                </Select>
            </div>

            <div class="block">
                <Select>
                    <option>{"Go"}</option>
                    <option>{"C"}</option>
                    <option selected={true}>{"Rust"}</option>
                    <option>{"C++"}</option>
                </Select>
            </div>

            <div class="block">
                <Select disable={true}>
                    <option>{"Go"}</option>
                    <option>{"C"}</option>
                    <option>{"Rust"}</option>
                    <option>{"C++"}</option>
                </Select>
            </div>

            <div class="block">
                <Select round={true}>
                    <option>{"Go"}</option>
                    <option>{"C"}</option>
                    <option>{"Rust"}</option>
                    <option>{"C++"}</option>
                </Select>
            </div>

            <div class="block">
                <Select color={Color::Primary}>
                    <option>{"Go"}</option>
                    <option>{"C"}</option>
                    <option>{"Rust"}</option>
                    <option>{"C++"}</option>
                </Select>
            </div>

            <div class="block">
                <Select size={Size::Large}>
                    <option>{"Go"}</option>
                    <option>{"C"}</option>
                    <option>{"Rust"}</option>
                    <option>{"C++"}</option>
                </Select>
            </div>

            <div class="block">
                <Select icon={"fas fa-envelope"} loading={true}>
                    <option>{"Go"}</option>
                    <option>{"C"}</option>
                    <option>{"Rust"}</option>
                    <option>{"C++"}</option>
                </Select>
            </div>
        </>
    }
}
