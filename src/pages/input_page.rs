use crate::components::{
    icon::FontAwesomeIcon,
    input::{Input, InputSizes},
};
use yew::{function_component, html};

use crate::components::input::{InputColors, InputTypes};

#[function_component(InputPage)]
pub fn input_page() -> Html {
    html! {
        <>
            <Input placeholder={"placeholder"} />

            <Input value={"i'm value"} />

            <Input value={"i'm value"} disable={true} />

            <Input value={"i'm value"} readonly={true} input_static={true} />

            <Input input_color={InputColors::Primary} placeholder={"Primary color"} />

            <Input input_type={InputTypes::Password} placeholder={"Password type"}/>

            <Input input_size={InputSizes::Large} placeholder={"Large size"}/>

            <Input placeholder={"placeholder"} control={true} loading={true} />

            <Input placeholder={"placeholder"} control={true} icon_left={true} icon_right={true}>
                <FontAwesomeIcon icon={"fas fa-envelope"} extra_class={"is-left"}/>
                <FontAwesomeIcon icon={"fas fa-check"} extra_class={"is-right"}/>
            </Input>
        </>
    }
}
