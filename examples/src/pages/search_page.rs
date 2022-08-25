use crate::api::list::search;
use rust_web_server_yew::{
    components::table::{Table, Td, Th},
    log,
};
use serde::Serialize;
use serde_json::json;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::{function_component, html, use_mut_ref, Callback, Html, TargetCast};
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[derive(Serialize, Clone, Default)]
pub struct Search {
    pub query: String,
}
#[function_component(SearchPage)]
pub fn search_page() -> Html {
    let search_value = use_mut_ref(Search::default);

    let search_async = {
        let search_value = search_value.clone();
        use_async_with_options(
            async move {
                let search_query = (*search_value.borrow()).clone();
                let result = search(search_query.query.as_str()).await;
                log!(
                    "search page query result",
                    json!(result.clone().unwrap()).to_string()
                );

                result
            },
            UseAsyncOptions::default(),
        )
    };

    let onclick = {
        let search_value = search_value.clone();
        let search_async = search_async.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let search = (*search_value).clone();
            let json_str = json!(search);
            log!("search page query", json_str.to_string());
            search_async.run();
        })
    };

    let oninput = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            *search_value.borrow_mut() = Search {
                query: input.value(),
            };
        })
    };

    let (data, count) = if let Some(list_info) = &search_async.data {
        let list = list_info.list.clone();

        (list, list_info.count)
    } else {
        (vec![], 0)
    };

    // table
    let thead_html = {
        html! {
            <tr>
                <Th>{"A"}</Th>
                <Th>{"B"}</Th>
                <Th centered=true>{"C"}</Th>
            </tr>
        }
    };

    let table_html = {
        data.iter()
            .map(|item| {
                html! {
                    <tr>
                        <Td>
                            <span class="tag">{item.a.clone()}</span>
                        </Td>
                        <Td>
                            {item.b.clone()}
                        </Td>
                        <Td centered=true>
                            {item.c}
                        </Td>
                    </tr>
                }
            })
            .collect::<Html>()
    };

    html! {
        <>
            <div class="field has-addons">
                <div class="control">
                    <input {oninput} class="input" type="text" placeholder="Find a repository" />
                </div>
                <div class="control">
                    <a class="button is-info" {onclick}>
                        {"Search"}
                    </a>
                </div>
            </div>

            <Table>
                { thead_html }
                {
                    if search_async.loading {
                        html! {
                            <tr>
                                <Td colspan=3 centered=true>{ "Loading..." }</Td>
                            </tr>
                        }
                    } else if count == 0 {
                        html! {
                            <tr>
                                <Td colspan=3 centered=true>{ "No data" }</Td>
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
