use crate::components::button::{
    Button, ButtonAddons, ButtonColors, ButtonGroup, Buttons, ButtonsSize,
};
use web_sys::MouseEvent;
use yew::{function_component, html, Callback};

#[function_component(ButtonPage)]
pub fn button_page() -> Html {
    let callback = {
        Callback::from(|e: MouseEvent| {
            e.prevent_default();
            gloo_console::log!("click");
        })
    };

    html! {
        <>
            <Button color={ButtonColors::Link} {callback}>{"Submit"}</Button>
            <Button color={ButtonColors::Link} light={true}>{"Cancel"}</Button>

            <Buttons size={ButtonsSize::Large}>
                <Button color={ButtonColors::Link}>{"Submit"}</Button>
                <Button color={ButtonColors::Link} light={true}>{"Cancel"}</Button>
            </Buttons>

            <ButtonGroup>
                <Button color={ButtonColors::Link}>{"Submit"}</Button>
                <Button color={ButtonColors::Link} light={true}>{"Cancel"}</Button>
            </ButtonGroup>

            <ButtonAddons>
                <Button>
                    <span class="icon is-small">
                        <i class="fas fa-align-left"></i>
                    </span>
                    <span>{"Left"}</span>
                </Button>
                <Button color={ButtonColors::Primary}>
                    <span class="icon is-small">
                        <i class="fas fa-align-center"></i>
                    </span>
                    <span>{"Center"}</span>
                </Button>
                <Button>
                    <span class="icon is-small">
                        <i class="fas fa-align-right"></i>
                    </span>
                    <span>{"Right"}</span>
                </Button>
            </ButtonAddons>
        </>
    }
}
