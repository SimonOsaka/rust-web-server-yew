use std::cmp::PartialEq;
use yew::{function_component, html, Children, Classes, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let TableProps { children } = props.clone();

    html! {
        <table class="table is-bordered is-striped is-narrow is-hoverable is-fullwidth">
            <tbody>
                { children.iter().collect::<Html>() }
            </tbody>
        </table>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TdProps {
    #[prop_or(false)]
    pub centered: bool,

    #[prop_or(None)]
    pub colspan: Option<u8>,

    #[prop_or("".into())]
    pub td_class: String,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(Td)]
pub fn td(props: &TdProps) -> Html {
    let TdProps {
        centered,
        children,
        td_class,
        colspan,
    } = props.clone();

    let mut td_classes = Classes::new();
    td_classes.push(td_class);
    if centered {
        td_classes.push("has-text-centered");
    }

    let colspan = colspan.map(|x| x.to_string());

    html! {
        <td class={td_classes} {colspan}>
            { children.iter().collect::<Html>() }
        </td>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ThProps {
    #[prop_or(false)]
    pub centered: bool,

    #[prop_or(None)]
    pub width: Option<u16>,

    #[prop_or("".into())]
    pub th_class: String,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(Th)]
pub fn th(props: &ThProps) -> Html {
    let ThProps {
        centered,
        th_class,
        children,
        width,
    } = props.clone();

    let mut th_classes = Classes::new();
    th_classes.push(th_class);
    if centered {
        th_classes.push("has-text-centered");
    }

    let style = width.map(|width| format!("width: {}px;", width));

    html! {
        <th class={th_classes} {style}>
            { children.iter().collect::<Html>() }
        </th>
    }
}
