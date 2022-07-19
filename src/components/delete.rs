use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Classes, Properties};

use super::Size;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or("".into())]
    pub extra_class: String,

    #[prop_or(Size::Default)]
    pub size: Size,

    #[prop_or_default]
    pub callback: Callback<MouseEvent>,
}
#[function_component(Delete)]
pub fn delete(props: &ButtonProps) -> Html {
    let ButtonProps {
        extra_class,
        size,
        callback,
    } = props.clone();

    let mut delete_class = Classes::new();
    delete_class.push("delete");
    delete_class.push(size);
    delete_class.push(extra_class);

    let callback = Callback::from(move |e: MouseEvent| {
        callback.emit(e);
    });

    html! {
        <button class={delete_class} onclick={callback}></button>
    }
}
