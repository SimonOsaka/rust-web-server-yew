use crate::bridge::notification_agent::{NotificationAgent, NotificationResponse};
use gloo::timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::{classes, function_component, html, use_state, Callback, Properties};
use yew_agent::use_bridge;

#[function_component(Notifications)]
pub fn notifications() -> Html {
    let notification_list = use_state(NotificationList::new);

    {
        let notification_list = notification_list.clone();
        use_bridge::<NotificationAgent, _>(move |out| {
            match out {
                NotificationResponse::Out(ns) => {
                    gloo_console::log!("Notification NotificationResponse...");

                    notification_list.set(ns);
                }
            }
            gloo_console::log!("Notifications received ...");
        });
    }

    let notification_html = {
        let notification_list = notification_list.clone();
        let list = (*notification_list).clone();
        list.into_iter().map(|(i, props)| {
            html! { <Notification key={i} ..props /> }
        })
    };

    html! {
        <div class="notification-area">
            { for notification_html }
        </div>
    }
}

pub type NotificationList = Vec<(usize, NotificationProps)>;

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

    let animation_class = use_state(|| "right-in");
    let to_close = use_state(|| false);
    let to_timeout = use_state(|| false);

    if *timeout != 0 {
        let timeout = timeout.clone();
        let to_timeout = to_timeout.clone();
        let animation_class = animation_class.clone();
        spawn_local(async move {
            TimeoutFuture::new(timeout).await;
            gloo_console::log!("Notification timeout future...");
            to_timeout.set(true);
            animation_class.set("right-out");
        });
    }

    let onclick = {
        let animation_class = animation_class.clone();
        let to_close = to_close.clone();
        Callback::once(move |_| {
            gloo_console::log!("Notification close...");
            to_close.set(true);
            animation_class.set("right-out");
        })
    };

    let onanimationend = {
        let close_callback = close_callback.clone();
        let timeout_callback = timeout_callback.clone();
        let to_close = to_close.clone();
        let to_timeout = to_timeout.clone();
        Callback::once(move |_| {
            if *to_close {
                gloo_console::log!("Notification onanimationend close...");
                close_callback.emit(());
            } else if *to_timeout {
                gloo_console::log!("Notification onanimationend timeout...");
                timeout_callback.emit(());
            }
        })
    };

    let text = text.clone();
    html! {
        <div class={classes!("notification", "default-style", *animation_class, class_type)} {onanimationend}>
            <button class="delete" {onclick}></button>
            { text }
        </div>
    }
}
