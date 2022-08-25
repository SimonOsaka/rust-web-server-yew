use gloo::timers::future::TimeoutFuture;
use rust_web_server_yew::{
    bridge::loading_agent::{LoadingAgent, LoadingInput},
    components::loading::LoadingProps,
};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, Callback, Properties};
use yew_agent::use_bridge;

#[function_component(LoadingPage)]
pub fn loading_page() -> Html {
    let loading_agent = use_bridge::<LoadingAgent, _>(|_| {});

    let onclick = {
        Callback::from(move |_| {
            let close_loading_agent = loading_agent.clone();
            spawn_local(async move {
                TimeoutFuture::new(3000).await;
                let loading_props = LoadingProps::builder().loading(false).build();
                close_loading_agent.send(LoadingInput::Input(loading_props));
            });

            let loading_props = LoadingProps::builder().loading(true).build();

            loading_agent.send(LoadingInput::Input(loading_props));
        })
    };

    html! {
        <>
            <button class="button is-primary" {onclick}>{"Loading"}</button>
        </>
    }
}
