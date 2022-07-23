use web_sys::MouseEvent;
use yew::{function_component, Callback, Properties};
use yew::{html, ChildrenWithProps, Html};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct BreadCrumbProps {
    #[prop_or("".into())]
    pub title: String,
    #[prop_or(None)]
    pub link: Option<String>,
}

#[function_component(BreadCrumb)]
pub fn breadcrumb(props: &BreadCrumbProps) -> Html {
    let BreadCrumbProps { title, link } = props.clone();

    if let Some(link) = link {
        html! {
            <li>
                <a href={link}>{title}</a>
            </li>
        }
    } else {
        let onclick = Callback::from(move |e: MouseEvent| {
            e.prevent_default();
        });

        html! {
            <li class="is-active">
                <a href="#" aria-current="page" {onclick}>{title}</a>
            </li>
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BreadCrumbsProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<BreadCrumb>,
}
#[function_component(BreadCrumbs)]
pub fn breadcrumbs(props: &BreadCrumbsProps) -> Html {
    let BreadCrumbsProps { children } = props.clone();

    html! {
        <nav class="breadcrumb" aria-label="breadcrumbs">
            <ul>
                { children.iter().collect::<Html>() }
            </ul>
        </nav>
    }
}
