use gloo::timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::{classes, function_component, html, Callback, Properties};

#[derive(Clone, Debug, PartialEq)]
pub enum NotificationShowType {
    Success,
    Warning,
    Danger,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NotificationProps {
    #[prop_or(3000)]
    pub timeout: u32,
    #[prop_or(NotificationShowType::Success)]
    pub show_type: NotificationShowType,
    pub text: String,
    #[prop_or_default]
    pub timeout_callback: Callback<()>,
    #[prop_or_default]
    pub close_callback: Callback<()>,
}

#[function_component(Notification)]
pub fn notification(props: &NotificationProps) -> Html {
    let NotificationProps {
        timeout,
        text,
        timeout_callback,
        show_type,
        close_callback,
    } = props;

    let class_type = match show_type {
        NotificationShowType::Success => "is-success",
        NotificationShowType::Warning => "is-warning",
        NotificationShowType::Danger => "is-danger",
    };

    if *timeout != 0 {
        let timeout = timeout.clone();
        let timeout_callback = timeout_callback.clone();
        spawn_local(async move {
            TimeoutFuture::new(timeout).await;
            gloo_console::log!("Notification timeout future...");

            timeout_callback.emit(());
        });
    }

    let onclick = {
        let close_callback = close_callback.clone();
        Callback::once(move |_| {
            gloo_console::log!("Notification close...");
            close_callback.emit(());
        })
    };

    let text = text.clone();
    html! {
        <div class={classes!("notification", "default-style", class_type)}>
            <button class="delete" {onclick}></button>
            { text }
        </div>
    }
}
