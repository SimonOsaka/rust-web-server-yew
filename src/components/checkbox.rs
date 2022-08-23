use yew::{function_component, html, use_state, Callback, Children, Classes, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CheckboxProps {
    #[prop_or(false)]
    pub init_checked: bool,

    #[prop_or(false)]
    pub disable: bool,

    #[prop_or("".into())]
    pub value: String,

    #[prop_or("".into())]
    pub extra_class: String,

    #[prop_or_default]
    pub callback: Callback<(bool, String)>,

    #[prop_or_default]
    pub children: Children,
}
#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
        disable,
        children,
        init_checked,
        extra_class,
        callback,
        value,
    } = props.clone();

    let state = use_state(|| init_checked);

    let mut checkbox_class = Classes::new();
    checkbox_class.push("checkbox");
    checkbox_class.push(extra_class);

    let onclick = {
        let value = value.clone();
        let state = state.clone();

        Callback::from(move |_| {
            let checked = !*state;
            state.set(checked);
            callback.emit((checked, value.clone()));
        })
    };

    html! {
        <label class={checkbox_class} disabled={disable}>
            <input type="checkbox" {onclick} checked={*state} {value} disabled={disable} />
            {children.iter().collect::<Html>()}
        </label>
    }
}
