use web_sys::{HtmlInputElement, InputEvent};
use yew::{
    function_component, html, Callback, ChildrenWithProps, Classes, Html, Properties, TargetCast,
};

use super::{icon::FontAwesomeIcon, Color, Size};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct InputProps {
    #[prop_or(String::from(""))]
    pub value: String,
    #[prop_or(InputTypes::Text)]
    pub input_type: InputTypes,
    #[prop_or(Color::Default)]
    pub color: Color,
    #[prop_or(Size::Default)]
    pub size: Size,
    #[prop_or(String::from(""))]
    pub placeholder: String,
    #[prop_or(false)]
    pub disable: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or(false)]
    pub input_static: bool,
    #[prop_or(false)]
    pub control: bool,
    #[prop_or(false)]
    pub loading: bool,
    #[prop_or(false)]
    pub round: bool,
    #[prop_or(false)]
    pub icon_left: bool,
    #[prop_or(false)]
    pub icon_right: bool,
    #[prop_or_default]
    pub children: ChildrenWithProps<FontAwesomeIcon>,
    #[prop_or_default]
    pub callback: Callback<String>,
}
#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let InputProps {
        value,
        input_type,
        color,
        size,
        placeholder,
        disable,
        readonly,
        input_static,
        control,
        loading,
        round,
        icon_left,
        icon_right,
        children,
        callback,
    } = props.clone();

    let mut input_class = Classes::new();
    input_class.push("input");
    input_class.push(color);
    input_class.push(size.clone());
    if input_static {
        input_class.push("is-static");
    }
    if round {
        input_class.push("is-rounded");
    }

    let oninput = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        callback.emit(input.value());
    });

    let input = html! {
        <input class={input_class} type={input_type.get()} {placeholder} {value} disabled={disable} {readonly}
        {oninput}/>
    };

    if control {
        let mut control_class = Classes::new();
        control_class.push("control");
        control_class.push(size);

        if loading {
            control_class.push("is-loading");
        }

        if icon_left {
            control_class.push("has-icons-left");
        }
        if icon_right {
            control_class.push("has-icons-right");
        }

        html! {
          <div class={control_class}>
              {input}
              {children.iter().collect::<Html>()}
          </div>
        }
    } else {
        input
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum InputTypes {
    Text,
    Password,
    Email,
    Tel,
}
impl InputTypes {
    pub fn get(&self) -> String {
        match self {
            InputTypes::Text => "text",
            InputTypes::Password => "password",
            InputTypes::Email => "email",
            InputTypes::Tel => "tel",
        }
        .to_string()
    }
}
