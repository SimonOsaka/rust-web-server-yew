use crate::bridge::loading_agent::{LoadingAgent, LoadingResponse};
use yew::{function_component, html, use_state, Classes, Properties};
use yew_agent::use_bridge;

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

#[function_component(Loadings)]
pub fn loadings() -> Html {
    let loading = use_state(|| false);

    {
        let loading = loading.clone();
        use_bridge::<LoadingAgent, _>(move |out| {
            match out {
                LoadingResponse::Out(is_loading) => {
                    gloo_console::log!("Loadings LoadingResponse...", is_loading);

                    loading.set(is_loading);
                }
            }
            gloo_console::log!("Loadings received ...");
        })
    };

    html! { <Loading loading={*loading} /> }
}
