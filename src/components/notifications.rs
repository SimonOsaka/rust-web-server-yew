use super::notification::NotificationProps;
use crate::{
    bridge::notification_agent::{NotificationAgent, NotificationResponse},
    components::notification::Notification,
};
use yew::{function_component, html, use_state};
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
