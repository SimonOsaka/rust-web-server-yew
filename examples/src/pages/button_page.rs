use rust_web_server_yew::{
    components::{
        button::{Button, ButtonAddons, ButtonGroup, Buttons, ButtonsSize},
        icon::{FontAwesomeIcon, FontAwesomeIconText},
        Color,
    },
    log,
};
use web_sys::MouseEvent;
use yew::{function_component, html, Callback};

#[function_component(ButtonPage)]
pub fn button_page() -> Html {
    let callback = {
        Callback::from(|e: MouseEvent| {
            e.prevent_default();
            log!("click");
        })
    };

    html! {
        <>
            <div class="block">
                <Button color={Color::Link} {callback}>{"Submit"}</Button>
                <Button color={Color::Link} light={true}>{"Cancel"}</Button>
            </div>

            <div class="block">
                <Buttons size={ButtonsSize::Large}>
                    <Button color={Color::Link}>{"Submit"}</Button>
                    <Button color={Color::Link} light={true}>{"Cancel"}</Button>
                </Buttons>
            </div>

            <div class="block">
                <ButtonGroup>
                    <Button color={Color::Link}>{"Submit"}</Button>
                    <Button color={Color::Link} light={true}>{"Cancel"}</Button>
                </ButtonGroup>
            </div>

            <div class="block">
                <ButtonAddons>
                    <Button>
                        <FontAwesomeIconText text={"Left"}>
                            <FontAwesomeIcon icon={"fas fa-align-left"} extra_class={"is-small"} />
                        </FontAwesomeIconText>
                    </Button>
                    <Button color={Color::Primary}>
                        <FontAwesomeIconText text={"Center"}>
                            <FontAwesomeIcon icon={"fas fa-align-center"} extra_class={"is-small"} />
                        </FontAwesomeIconText>
                    </Button>
                    <Button>
                        <FontAwesomeIconText text={"Right"}>
                            <FontAwesomeIcon icon={"fas fa-align-right"} extra_class={"is-small"} />
                        </FontAwesomeIconText>
                    </Button>
                </ButtonAddons>
            </div>
        </>
    }
}
