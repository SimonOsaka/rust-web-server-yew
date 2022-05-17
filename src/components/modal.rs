use web_sys::MouseEvent;
use yew::{
    classes, function_component, html, use_effect_with_deps, use_state, Callback, Children,
    Properties,
};

pub enum ModalEnum {
    Cancel,
    Ok,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ModalProps {
    #[prop_or(false)]
    pub is_active: bool,
    #[prop_or("Ok".to_string())]
    pub btn_ok_text: String,
    #[prop_or("Cancel".to_string())]
    pub btn_cancel_text: String,
    #[prop_or_default]
    pub children: Children,
    pub callback: Callback<ModalEnum>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let ModalProps {
        is_active,
        callback,
        children,
        btn_ok_text,
        btn_cancel_text,
    } = props;

    let state = use_state(|| false);

    // let is_active_bool = props.is_active;
    {
        let state = state.clone();
        use_effect_with_deps(
            move |active| {
                state.set(*active);
                || ()
            },
            *is_active,
        );
    }

    let is_active_class = if *state { "is-active" } else { "" };

    let onclick_close = {
        let state = state.clone();
        let callback = callback.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            state.set(false);
            callback.emit(ModalEnum::Cancel);
        })
    };

    let onclick_ok = {
        let state = state.clone();
        let callback = callback.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            state.set(false);
            callback.emit(ModalEnum::Ok);
        })
    };

    html! {
        <div class={classes!("modal", is_active_class)}>
            <div class="modal-background" onclick={onclick_close.clone()}></div>
            <div class="modal-content">
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"Modal title"}</p>
                        <button class="delete" aria-label="close" onclick={onclick_close.clone()}></button>
                    </header>
                    <section class="modal-card-body">
                        { for children.iter() }
                    </section>
                    <footer class="modal-card-foot">
                        <button class="button is-success" onclick={onclick_ok}>{ btn_ok_text }</button>
                        <button class="button" onclick={onclick_close.clone()}>{ btn_cancel_text }</button>
                    </footer>
                </div>
            </div>
            <button class="modal-close is-large" aria-label="close" onclick={onclick_close}></button>
        </div>
    }
}
