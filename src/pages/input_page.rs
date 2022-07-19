use crate::components::{icon::FontAwesomeIcon, input::Input, Color, Size};
use yew::{function_component, html, Callback};

use crate::components::input::InputTypes;

#[function_component(InputPage)]
pub fn input_page() -> Html {
    let callback = Callback::from(|str: String| {
        gloo_console::log!(str);
    });

    html! {
        <>
            <div class="block">
                <Input placeholder={"placeholder"} {callback} />
            </div>

            <div class="block">
                <Input value={"i'm value"} />
            </div>

            <div class="block">
                <Input value={"i'm value"} disable={true} />
            </div>

            <div class="block">
                <Input value={"i'm value"} readonly={true} input_static={true} />
            </div>

            <div class="block">
                <Input color={Color::Primary} placeholder={"Primary color"} />
            </div>

            <div class="block">
                <Input input_type={InputTypes::Password} placeholder={"Password type"}/>
            </div>

            <div class="block">
                <Input size={Size::Large} placeholder={"Large size"}/>
            </div>

            <div class="block">
                <Input placeholder={"placeholder"} control={true} loading={true} />
            </div>

            <div class="block">
                <Input placeholder={"placeholder"} control={true} icon_left={true} icon_right={true}>
                    <FontAwesomeIcon icon={"fas fa-envelope"} extra_class={"is-left"}/>
                    <FontAwesomeIcon icon={"fas fa-check"} extra_class={"is-right"}/>
                </Input>
            </div>
        </>
    }
}
