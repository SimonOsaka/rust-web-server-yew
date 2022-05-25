use crate::components::loading::Loading;
use gloo::timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_state, Callback};

#[function_component(LoadingPage)]
pub fn loading_page() -> Html {
    let loading = use_state(|| false);

    let onclick = {
        let loading = loading.clone();
        Callback::from(move |_| {
            let loading_open = loading.clone();
            let loading_close = loading.clone();
            spawn_local(async move {
                TimeoutFuture::new(3000).await;
                loading_close.set(false);
            });
            loading_open.set(true);
        })
    };

    html! {
        <>
            <button class="is-primary" {onclick}>{"Loading"}</button>
            <Loading loading={*loading}/>
        </>
    }
}
