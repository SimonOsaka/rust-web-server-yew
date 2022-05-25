use std::cmp::PartialEq;

use serde::Serialize;

use yew::{function_component, html, ChildrenWithProps, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TableProps<T>
where
    T: PartialEq + Serialize + TableData,
{
    #[prop_or(vec![])]
    pub data: Vec<T>,
    #[prop_or_default]
    pub children: ChildrenWithProps<Td>,
}

#[function_component(Table)]
pub fn table<T>(props: &TableProps<T>) -> Html
where
    T: PartialEq + Serialize + TableData,
{
    let TableProps { data, children } = props.clone();

    // all
    let children = children.clone();
    let all = children
        .iter()
        .map(|child| {
            let label = &child.props.label;
            let name = &child.props.name;
            let width = child.props.width;
            let centered = child.props.centered;
            (label.into(), name.into(), width, centered)
        })
        .collect::<Vec<(String, String, Option<u16>, bool)>>();

    let thead_html = {
        let hds = all.clone();
        html! {
            <tr>
            {
                hds.iter().map(|(th, _, width, centered)| {
                    let style = if let Some(w) = width {
                        format!("width:{w}px;")
                    } else {
                        "".to_string()
                    };
                    let class = if *centered {
                        "has-text-centered"
                    } else {
                        ""
                    };
                    html! {<th {class} {style}>{th}</th>}
                }).collect::<Html>()
            }
            </tr>
        }
    };

    gloo_console::log!("Table thead ...");

    // tbody_html
    let tbody_html = {
        if data.len() == 0 {
            html! {<tr><td colspan={format!("{}", all.len())} class="has-text-centered">{"No data..."}</td></tr>}
        } else {
            let d = data.clone();
            html! {
                {
                    for d.iter().map(move |item| {
                        let all = all.clone();
                        let value = serde_json::to_value(item).unwrap();
                        html!{
                            <tr>
                                {
                                    for all.into_iter().map(|(_, td_name, _, centered)| {
                                        let class = if centered {
                                            "has-text-centered"
                                        } else {
                                            ""
                                        };
                                        if let Some(field_html) = item.get_field_as_html(&td_name) {
                                           html!{ <td {class}>{ field_html }</td> }
                                        } else if let Some(td_value) = value.get(td_name) {
                                                let tdv = (*td_value).clone();
                                                let field_value = if tdv.is_i64() {
                                                    tdv.as_i64().unwrap().to_string()
                                                } else if tdv.is_string() {
                                                    tdv.as_str().unwrap().to_string()
                                                } else if tdv.is_boolean(){
                                                    tdv.as_bool().unwrap().to_string()
                                                } else {
                                                    "".to_string()
                                                };

                                                html! { <td {class}>{ field_value }</td> }
                                            } else {
                                                html! { <td></td> }
                                            }
                                    })
                                }
                            </tr>
                        }
                    })
                }
            }
        }
    };

    gloo_console::log!("Table tbody ...");

    html! {
        <table class="table is-bordered is-striped is-narrow is-hoverable is-fullwidth">
            <thead>
                { thead_html }
            </thead>
            <tbody>
                { tbody_html }
            </tbody>
        </table>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TdProps {
    #[prop_or("".to_string())]
    pub name: String,
    #[prop_or("".to_string())]
    pub label: String,
    #[prop_or(None)]
    pub width: Option<u16>,
    #[prop_or(false)]
    pub centered: bool,
}

#[function_component(Td)]
pub fn td(_props: &TdProps) -> Html {
    html! {}
}

pub trait TableData: 'static + Default + Clone + Serialize {
    fn get_field_as_html(&self, field_name: &str) -> Option<Html>;

    // fn get_field_as_value(&self, field_name: &str) -> Option<String>;
}
