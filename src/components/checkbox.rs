use yew::{function_component, html, Callback, Children, Classes, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CheckboxProps {
    #[prop_or(false)]
    pub check: bool,

    #[prop_or(false)]
    pub disable: bool,

    #[prop_or("".to_string())]
    pub extra_class: String,

    #[prop_or_default]
    pub callback: Callback<bool>,

    #[prop_or_default]
    pub children: Children,
}
#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
        disable,
        children,
        check,
        extra_class,
        callback,
    } = props.clone();

    let mut checkbox_class = Classes::new();
    checkbox_class.push("checkbox");
    checkbox_class.push(extra_class);

    let onclick = {
        Callback::from(move |_| {
            callback.emit(!check);
        })
    };

    html! {
        <label class={checkbox_class} disabled={disable}>
            <input type="checkbox" {onclick} checked={check} disabled={disable} />
            {children.iter().collect::<Html>()}
        </label>
    }
}
