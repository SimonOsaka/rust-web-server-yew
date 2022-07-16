use yew::{function_component, html, use_state, Callback, Children, Classes, Html, Properties};

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

    let state = use_state(|| check);

    let mut checkbox_class = Classes::new();
    checkbox_class.push("checkbox");
    checkbox_class.push(extra_class);

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            let checked = *state.clone();
            state.set(!checked);
            callback.emit(!checked);
        })
    };

    html! {
        <label class={checkbox_class} disabled={disable}>
            <input type="checkbox" {onclick} checked={*state} disabled={disable} />
            {children.iter().collect::<Html>()}
        </label>
    }
}
