use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Children, Classes, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or(ButtonColors::Default)]
    pub color: ButtonColors,
    #[prop_or(ButtonSize::Default)]
    pub size: ButtonSize,
    #[prop_or(false)]
    pub disable: bool,
    #[prop_or(false)]
    pub loading: bool,
    #[prop_or(false)]
    pub focus: bool,
    #[prop_or(false)]
    pub active: bool,
    #[prop_or(false)]
    pub hover: bool,
    #[prop_or(false)]
    pub round: bool,
    #[prop_or(false)]
    pub outline: bool,
    #[prop_or(false)]
    pub light: bool,
    #[prop_or_default]
    pub callback: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        callback,
        disable,
        loading,
        focus,
        active,
        hover,
        round,
        outline,
        color,
        light,
        size,
        children,
    } = props.clone();

    let mut cls = Classes::new();

    cls.push("button");
    let color = match color {
        ButtonColors::Primary => "is-primary",
        ButtonColors::Danger => "is-danger",
        ButtonColors::Info => "is-info",
        ButtonColors::Link => "is-link",
        ButtonColors::Success => "is-success",
        ButtonColors::Warning => "is-warning",
        ButtonColors::Default => "",
    };
    cls.push(color);

    let size = match size {
        ButtonSize::Small => "is-small",
        ButtonSize::Normal => "is-normal",
        ButtonSize::Medium => "is-medium",
        ButtonSize::Large => "is-large",
        ButtonSize::Default => "",
    };
    cls.push(size);

    if loading {
        cls.push("is-loading");
    }
    if active {
        cls.push("is-active");
    }
    if focus {
        cls.push("is-focused");
    }
    if hover {
        cls.push("is-hovered");
    }
    if round {
        cls.push("is-rounded");
    }
    if outline {
        cls.push("is-outlined");
    }
    if light {
        cls.push("is-light");
    }

    let onclick = {
        Callback::from(move |e: MouseEvent| {
            callback.emit(e);
        })
    };

    html! {
        <button class={ cls } { onclick } disabled={ disable }>{ for children }</button>
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ButtonColors {
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    Default,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ButtonSize {
    Small,
    Normal,
    Medium,
    Large,
    Default,
}
