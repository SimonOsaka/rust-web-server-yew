use crate::components::card::Card;
use crate::components::media_object::MediaObjectArticle;
use yew::{classes, function_component, html, Properties};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub is_active: bool,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    let is_active_bool = props.is_active;

    let is_active_class = if is_active_bool { "is-active" } else { "" };

    html! {
        <div class={classes!("modal", is_active_class)}>
        <div class="modal-background"></div>
        <div class="modal-content">
            <div class="box">
            <MediaObjectArticle/>
            </div>
        </div>
        <button class="modal-close is-large" aria-label="close"></button>
        </div>
    }
}

#[function_component(ModalImage)]
pub fn modal_image(props: &Props) -> Html {
    let is_active_bool = props.is_active;

    let is_active_class = if is_active_bool { "is-active" } else { "" };

    html! {
        <div class={classes!("modal", is_active_class)}>
        <div class="modal-background"></div>
        <div class="modal-content">
            <p class="image is-4by3">
            <img src="https://bulma.io/images/placeholders/1280x960.png" alt=""/>
            </p>
        </div>
        <button class="modal-close is-large" aria-label="close"></button>
        </div>
    }
}

#[function_component(ModalCard)]
pub fn modal_card(props: &Props) -> Html {
    let is_active_bool = props.is_active;

    let is_active_class = if is_active_bool { "is-active" } else { "" };

    html! {
        <div class={classes!("modal", is_active_class)}>
            <div class="modal-background"></div>
            <div class="modal-card">
                <header class="modal-card-head">
                <p class="modal-card-title">{"Modal title"}</p>
                <button class="delete" aria-label="close"></button>
                </header>
                <section class="modal-card-body">
                <Card/>
                </section>
                <footer class="modal-card-foot">
                <button class="button is-success">{"Save changes"}</button>
                <button class="button">{"Cancel"}</button>
                </footer>
            </div>
        </div>
    }
}
