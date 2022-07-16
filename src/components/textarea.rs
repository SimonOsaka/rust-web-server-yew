use web_sys::{HtmlTextAreaElement, InputEvent};
use yew::{
    function_component, html, Callback, ChildrenWithProps, Classes, Html, Properties, TargetCast,
};

use super::icon::FontAwesomeIcon;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TextareaProps {
    #[prop_or(String::from(""))]
    pub value: String,
    #[prop_or(2)]
    pub rows: u8,
    #[prop_or(String::from(""))]
    pub placeholder: String,
    #[prop_or(TextareaColors::Default)]
    pub color: TextareaColors,
    #[prop_or(TextareaSizes::Default)]
    pub size: TextareaSizes,
    #[prop_or(false)]
    pub disable: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or(false)]
    pub control: bool,
    #[prop_or(false)]
    pub loading: bool,
    #[prop_or(false)]
    pub resize: bool,
    #[prop_or_default]
    pub children: ChildrenWithProps<FontAwesomeIcon>,
    #[prop_or_default]
    pub callback: Callback<String>,
}
#[function_component(Textarea)]
pub fn input(props: &TextareaProps) -> Html {
    let TextareaProps {
        value,
        rows,
        placeholder,
        color,
        size,
        disable,
        readonly,
        control,
        loading,
        children,
        resize,
        callback,
    } = props.clone();

    let mut textarea_class = Classes::new();
    textarea_class.push("textarea");
    textarea_class.push(color.get());
    textarea_class.push(size.get());
    if resize {
        textarea_class.push("has-fixed-size");
    }

    let oninput = Callback::from(move |e: InputEvent| {
        let input: HtmlTextAreaElement = e.target_unchecked_into();
        callback.emit(input.value());
    });

    let textarea = html! {
        <textarea class={textarea_class} {placeholder} rows={rows.to_string()} disabled={disable} {readonly} {oninput} {value}/>
    };

    if control {
        let mut control_class = Classes::new();
        control_class.push("control");
        control_class.push(size.get());

        if loading {
            control_class.push("is-loading");
        }

        html! {
          <div class={control_class}>
              {textarea}
              {children.iter().collect::<Html>()}
          </div>
        }
    } else {
        textarea
    }
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum TextareaColors {
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    Default,
}
impl TextareaColors {
    pub fn get(&self) -> String {
        match self {
            TextareaColors::Primary => "is-primary",
            TextareaColors::Link => "is-link",
            TextareaColors::Info => "is-info",
            TextareaColors::Success => "is-success",
            TextareaColors::Warning => "is-warning",
            TextareaColors::Danger => "is-danger",
            TextareaColors::Default => "",
        }
        .to_string()
    }
}

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum TextareaSizes {
    Small,
    Normal,
    Medium,
    Large,
    Default,
}
impl TextareaSizes {
    pub fn get(&self) -> String {
        match self {
            TextareaSizes::Small => "is-small",
            TextareaSizes::Normal => "is-normal",
            TextareaSizes::Medium => "is-medium",
            TextareaSizes::Large => "is-large",
            TextareaSizes::Default => "",
        }
        .to_string()
    }
}
