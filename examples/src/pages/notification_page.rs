use rust_web_server_yew::{
    bridge::notification_agent::{NotificationAgent, NotificationInput},
    components::notifications::{NotificationProps, NotificationShowType},
    log,
};
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};
use yew_agent::use_bridge;

#[function_component(NotificationPage)]
pub fn notification_page() -> Html {
    let notification_agent = use_bridge::<NotificationAgent, _>(|_| {});

    let onclick = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            log!("send notification");

            let props = NotificationProps::builder()
                .show_type(get_show_type())
                .text(format!("abc-{}", random_number::random!(0..100)))
                .timeout(3000)
                .build();

            notification_agent.send(NotificationInput::Add(props));
        })
    };

    html! {
        <>
            <button class="button is-info" {onclick}>{"Open"}</button>
        </>
    }
}

fn get_show_type() -> NotificationShowType {
    let t = random_number::random!(0..=2);
    match t {
        0 => NotificationShowType::Success,
        1 => NotificationShowType::Danger,
        2 => NotificationShowType::Warning,
        _ => panic!("get class type failed"),
    }
}
