use yew::{function_component, html, Children, ChildrenWithProps, Classes, Html, Properties};

use super::{Color, Size};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TagProps {
    #[prop_or(Color::Default)]
    pub color: Color,
    #[prop_or(Size::Default)]
    pub size: Size,
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
    cls.push(color);
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
