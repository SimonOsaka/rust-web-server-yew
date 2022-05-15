use yew::{function_component, html, Html, Properties};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct TableProps {
    pub head: Vec<String>,
    pub data: Vec<Vec<String>>,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let head = &props.head;
    let data = &props.data;

    let thead_html = html! {
        <tr>
        {
            head.iter().map(|th| {
                html! {<th>{th}</th>}
            }).collect::<Html>()
        }
        </tr>
    };

    let tbody_html = data.iter().map(|tr| {
        html! {
            <tr>
            {
                tr.iter().map(|td|{
                    html! {
                        <td>{td}</td>
                    }
                }).collect::<Html>()
            }
            </tr>
        }
    });

    html! {
        <table class="table is-bordered is-striped is-narrow is-hoverable is-fullwidth">
            <thead>
                {thead_html}
            </thead>
            <tbody>
                {for tbody_html}
            </tbody>
        </table>
    }
}
