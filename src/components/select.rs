use web_sys::{Event, HtmlSelectElement};
use yew::{function_component, html, Callback, Children, Classes, Html, Properties, TargetCast};

use super::icon::FontAwesomeIcon;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SelectProps {
    #[prop_or(SelectColors::Default)]
    pub color: SelectColors,
    #[prop_or(SelectSizes::Default)]
    pub size: SelectSizes,
    #[prop_or(false)]
    pub disable: bool,
    #[prop_or(false)]
    pub loading: bool,
    #[prop_or(false)]
    pub round: bool,
    #[prop_or("".to_string())]
    pub icon: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub callback: Callback<String>,
}
#[function_component(Select)]
pub fn input(props: &SelectProps) -> Html {
    let SelectProps {
        color,
        size,
        disable,
        loading,
        children,
        callback,
        round,
        icon,
    } = props.clone();

    let mut select_class = Classes::new();
    select_class.push("select");
    select_class.push(color.get());
    select_class.push(size.get());
    if round {
        select_class.push("is-rounded");
    }
    if loading {
        select_class.push("is-loading");
    }

    let onchange = Callback::from(move |e: Event| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        callback.emit(input.value());
    });

    let select = html! {
        <div class={select_class} >
            <select {onchange} disabled={disable}>
                {children.iter().collect::<Html>()}
            </select>
        </div>
    };

    if !icon.is_empty() {
        let mut control_class = Classes::new();
        control_class.push("control");
        control_class.push("has-icons-left");
        control_class.push(size.get());

        html! {
          <div class={control_class}>
              {select}
              <FontAwesomeIcon {icon} extra_class={"is-left"}/>
          </div>
        }
    } else {
        select
    }
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum SelectColors {
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    Default,
}
impl SelectColors {
    pub fn get(&self) -> String {
        match self {
            SelectColors::Primary => "is-primary",
            SelectColors::Link => "is-link",
            SelectColors::Info => "is-info",
            SelectColors::Success => "is-success",
            SelectColors::Warning => "is-warning",
            SelectColors::Danger => "is-danger",
            SelectColors::Default => "",
        }
        .to_string()
    }
}

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum SelectSizes {
    Small,
    Normal,
    Medium,
    Large,
    Default,
}
impl SelectSizes {
    pub fn get(&self) -> String {
        match self {
            SelectSizes::Small => "is-small",
            SelectSizes::Normal => "is-normal",
            SelectSizes::Medium => "is-medium",
            SelectSizes::Large => "is-large",
            SelectSizes::Default => "",
        }
        .to_string()
    }
}
