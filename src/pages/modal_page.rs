use crate::components::{
    card::Card,
    modal::{Modal, ModalEnum, ModalProps},
};
use web_sys::MouseEvent;
use yew::{function_component, html, props, use_state, Callback};

#[function_component(ModalPage)]
pub fn modal_page() -> Html {
    let modal_is_active = use_state(|| false);
    let onclick_button = {
        let modal_is_active = modal_is_active.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            modal_is_active.set(true);
        })
    };

    let callback = {
        let modal_is_active = modal_is_active.clone();
        Callback::from(move |ty| match ty {
            ModalEnum::Cancel => modal_is_active.set(false),
            ModalEnum::Ok => modal_is_active.set(false),
        })
    };

    let modal_props = props! {
        ModalProps {
            callback,
        }
    };

    html! {
        <>
            <button class="button is-primary" onclick={onclick_button}>{ "Open" }</button>
            <Modal is_active={*modal_is_active} ..modal_props>
                // <div class="box">
                //     <MediaObjectArticle />
                // </div>
                <Card/>
            </Modal>
        </>
    }
}
