use yew::{function_component, html, Properties};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct BreadCrumbProps {
    pub data: Vec<(String, bool)>,
}

#[function_component(BreadCrumb)]
pub fn breadcrumb(props: &BreadCrumbProps) -> Html {
    let vec = props.data.clone();

    let li_html = vec.into_iter().map(|(name, is_active)| {
        if is_active {
            html! {<li class="is-active"><a href="#" aria-current="page">{name}</a></li>}
        } else {
            html! {<li><a href="#">{name}</a></li>}
        }
    });

    html! {
        <nav class="breadcrumb" aria-label="breadcrumbs">
        <ul>
            {
                for li_html
            }
        </ul>
        </nav>
    }
}
