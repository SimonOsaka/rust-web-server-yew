use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct FontAwesomeIconProps {
    #[prop_or("".to_string())]
    pub icon: String,
    #[prop_or("".to_string())]
    pub extra_class: String,
}

#[function_component(FontAwesomeIcon)]
pub fn font_awesome_icon(props: &FontAwesomeIconProps) -> Html {
    let FontAwesomeIconProps { icon, extra_class } = props.clone();

    let mut cls = Classes::new();
    cls.push("icon");
    cls.push(extra_class);

    html! {
        <span class={cls}>
            <i class={icon}></i>
        </span>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FontAwesomeIconTextProps {
    #[prop_or("".to_string())]
    pub text: String,
    #[prop_or("".to_string())]
    pub extra_class: String,
    #[prop_or_default]
    pub children: ChildrenWithProps<FontAwesomeIcon>,
}

#[function_component(FontAwesomeIconText)]
pub fn font_awesome_icon_text(props: &FontAwesomeIconTextProps) -> Html {
    let FontAwesomeIconTextProps {
        text,
        children,
        extra_class,
    } = props.clone();

    let mut cls = Classes::new();
    cls.push("icon-text");
    cls.push(extra_class);

    html! {
        <span class={cls}>
            {for children}
            <span>{text}</span>
        </span>
    }
}
