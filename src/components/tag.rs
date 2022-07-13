use yew::{function_component, html, Children, ChildrenWithProps, Classes, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TagProps {
    #[prop_or(TagColors::Default)]
    pub color: TagColors,
    #[prop_or(TagSize::Default)]
    pub size: TagSize,
    #[prop_or(false)]
    pub round: bool,
    #[prop_or(false)]
    pub light: bool,
    #[prop_or(false)]
    pub delete: bool,
    #[prop_or("span".to_string())]
    pub tag_name: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Tag)]
pub fn tag(props: &TagProps) -> Html {
    let TagProps {
        color,
        size,
        round,
        light,
        children,
        tag_name,
        delete,
    } = props.clone();

    let mut cls = Classes::new();
    cls.push("tag");

    let color = match color {
        TagColors::Primary => "is-primary",
        TagColors::Danger => "is-danger",
        TagColors::Info => "is-info",
        TagColors::Link => "is-link",
        TagColors::Success => "is-success",
        TagColors::Warning => "is-warning",
        TagColors::Black => "is-black",
        TagColors::Dark => "is-dark",
        TagColors::Light => "is-light",
        TagColors::White => "is-white",
        TagColors::Default => "",
    };
    cls.push(color);

    let size = match size {
        TagSize::Normal => "is-normal",
        TagSize::Medium => "is-medium",
        TagSize::Large => "is-large",
        TagSize::Default => "",
    };
    cls.push(size);

    if round {
        cls.push("is-rounded");
    }
    if light {
        cls.push("is-light");
    }

    let delete_html = if delete {
        html! {
            <button class="delete is-small"></button>
        }
    } else {
        html! {}
    };

    html! {
        <@{tag_name} class={ cls }>
            { for children }
            { delete_html }
        </@>
    }
}

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum TagColors {
    Black,
    Dark,
    Light,
    White,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    Default,
}

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum TagSize {
    Normal,
    Medium,
    Large,
    Default,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TagsProps {
    #[prop_or(false)]
    pub addon: bool,
    #[prop_or(false)]
    pub delete: bool,
    #[prop_or_default]
    pub children: Children,
}
#[function_component(Tags)]
pub fn tags(props: &TagsProps) -> Html {
    let TagsProps {
        children,
        addon,
        delete,
    } = props.clone();

    let mut cls = Classes::new();
    cls.push("tags");

    if addon {
        cls.push("has-addons");
    }

    let delete_html = if delete {
        html! {
            <a class="tag is-delete"></a>
        }
    } else {
        html! {}
    };

    html! {
        <div class={cls}>
            { children.iter().collect::<Html>() }
            { delete_html }
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct GroupTagsProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Tags>,
}
#[function_component(GroupTags)]
pub fn group_tags(props: &GroupTagsProps) -> Html {
    let GroupTagsProps { children } = props.clone();

    let children_html = children
        .iter()
        .map(|x| {
            html! {
                <div class="control">
                    {x}
                </div>
            }
        })
        .collect::<Html>();
    html! {
        <div class="field is-grouped is-grouped-multiline">
            {children_html}
        </div>
    }
}
