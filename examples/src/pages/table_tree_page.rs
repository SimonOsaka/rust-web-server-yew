use crate::api::site;
use rust_web_server_yew::{
    components::table::{Table, Td, Th},
    log,
};
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html};
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[function_component(TableTreePage)]
pub fn table_tree_page() -> Html {
    let site_all = {
        use_async_with_options(
            async move {
                log!("async request site::all");

                site::all().await
            },
            UseAsyncOptions::enable_auto(),
        )
    };

    let data = if let Some(site_info) = &site_all.data {
        site_info.list.clone()
    } else {
        vec![]
    };

    // table props
    let thead_html = {
        html! {
            <tr>
                <Th>{"Id"}</Th>
                <Th>{"Domain"}</Th>
                <Th>{"Sub domain"}</Th>
                <Th centered=true width=200>{"Operation"}</Th>
            </tr>
        }
    };

    let table_html = {
        data.iter()
            .map(|item| {
                let onclick = {
                    let id = item.id;
                    Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        log!("view", id);
                    })
                };

                html! {
                    <tr>
                        <Td>
                            { &item.id }
                        </Td>
                        <Td>
                            {
                                if item.level == 0 {
                                    &item.domain
                                } else {
                                    ""
                                }
                            }
                        </Td>
                        <Td>
                            {
                                if item.level == 1 {
                                    &item.domain
                                } else {
                                    ""
                                }
                            }
                        </Td>
                        <Td centered=true>
                            <div class="buttons are-small">
                                <button class="button is-info is-outlined" {onclick}>{"view"}</button>
                                <button class="button is-link is-outlined">{"edit"}</button>
                                <button class="button is-danger is-outlined">{"delete"}</button>
                            </div>
                        </Td>
                    </tr>
                }
            })
            .collect::<Html>()
    };

    html! {
        <>
            <Table>
                { thead_html }
                {
                    if site_all.loading {
                        html! {
                            <tr>
                                <Td colspan=4 centered=true>{ "Loading..." }</Td>
                            </tr>
                        }
                    } else if data.is_empty() {
                        html! {
                            <tr>
                                <Td colspan=4 centered=true>{ "No data" }</Td>
                            </tr>
                        }
                    } else {
                        table_html
                    }
                }
            </Table>
        </>
    }
}
