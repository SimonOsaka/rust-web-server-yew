use web_sys::MouseEvent;
use yew::{
    function_component, html, Callback, Children, ChildrenWithProps, Classes, Html, Properties,
};

use super::{Color, Size};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or(Color::Default)]
    pub color: Color,
    #[prop_or(Size::Default)]
    pub size: Size,
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
    #[prop_or(false)]
    pub select: bool,
    #[prop_or(false)]
    pub responsive: bool,
    #[prop_or(false)]
    pub fullwidth: bool,
    #[prop_or(false)]
    pub invert: bool,
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
        select,
        responsive,
        fullwidth,
        invert,
    } = props.clone();

    let mut cls = Classes::new();

    cls.push("button");
    cls.push(color);
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
    if select {
        cls.push("is-selected");
    }
    if responsive {
        cls.push("is-responsive");
    }
    if fullwidth {
        cls.push("is-fullwidth");
    }
    if invert {
        cls.push("is-inverted");
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

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ButtonsProps {
    #[prop_or(false)]
    pub addon: bool,
    #[prop_or(ButtonsAlignment::Left)]
    pub alignment: ButtonsAlignment,
    #[prop_or(ButtonsSize::Default)]
    pub size: ButtonsSize,
    #[prop_or_default]
    pub children: ChildrenWithProps<Button>,
}
#[function_component(Buttons)]
pub fn buttons(props: &ButtonsProps) -> Html {
    let ButtonsProps {
        children,
        size,
        addon,
        alignment,
    } = props.clone();

    let mut cls = Classes::new();
    cls.push("buttons");

    let size = match size {
        ButtonsSize::Small => "are-small",
        ButtonsSize::Medium => "are-medium",
        ButtonsSize::Large => "are-large",
        ButtonsSize::Default => "",
    };
    cls.push(size);

    if addon {
        cls.push("has-addons");
    }

    let alignment = match alignment {
        ButtonsAlignment::Center => "is-centered",
        ButtonsAlignment::Right => "is-right",
        ButtonsAlignment::Left => "",
    };
    cls.push(alignment);

    let buttons_html = children.iter().map(|btn| html! {{btn}}).collect::<Html>();

    html! {
        <div class={cls}>
            {buttons_html}
        </div>
    }
}

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum ButtonsSize {
    Small,
    Medium,
    Large,
    Default,
}

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum ButtonsAlignment {
    Center,
    Right,
    Left,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ButtonGroupProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Button>,
}
#[function_component(ButtonGroup)]
pub fn button_group(props: &ButtonGroupProps) -> Html {
    let ButtonGroupProps { children } = props.clone();

    let button_group_html = children
        .iter()
        .map(|btn| {
            html! {
                <div class="control">
                    {btn}
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="field is-grouped">
            {button_group_html}
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ButtonAddonsProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Button>,
}
#[function_component(ButtonAddons)]
pub fn button_addons(props: &ButtonAddonsProps) -> Html {
    let ButtonAddonsProps { children } = props.clone();

    let button_addons_html = children
        .iter()
        .map(|btn| {
            html! {
                <div class="control">
                    {btn}
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="field has-addons">
            {button_addons_html}
        </div>
    }
}
