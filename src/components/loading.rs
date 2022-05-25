use yew::{function_component, html, Classes, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoadingProps {
    #[prop_or(false)]
    pub loading: bool,
}

#[function_component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    let loading = props.loading;
    let mut cls = Classes::new();
    cls.push("modal");
    if loading {
        cls.push("is-active");
    }

    gloo_console::log!("Loading ...", loading);

    html! {
        <div class={cls}>
            <div class="modal-background"></div>
            <div class="modal-content">
                <progress class="progress is-small is-primary" max="100"></progress>
            </div>
        </div>
    }
}
