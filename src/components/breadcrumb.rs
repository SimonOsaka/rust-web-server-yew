use yew::{function_component, html, Html, Properties};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct BreadCrumbProps {
    pub data: Vec<(String, bool)>,
}

#[function_component(BreadCrumb)]
pub fn breadcrumb(props: &BreadCrumbProps) -> Html {
    let BreadCrumbProps { data } = props.clone();

    let li_html = data
        .into_iter()
        .map(|(name, is_active)| {
            if is_active {
                html! {<li class="is-active"><a href="#" aria-current="page">{name}</a></li>}
            } else {
                html! {<li><a href="#">{name}</a></li>}
            }
        })
        .collect::<Html>();

    html! {
        <nav class="breadcrumb" aria-label="breadcrumbs">
            <ul>
                { li_html }
            </ul>
        </nav>
    }
}
