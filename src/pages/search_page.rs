use crate::{
    components::table::{Table, TableData, TableProps, Td},
    types::list::ListInfo,
};
use gloo::console;
use serde::{Deserialize, Serialize};
use serde_json::json;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::{function_component, html, props, use_state, Callback, TargetCast};
use yew_hooks::use_async;

use crate::api::list::search;

#[derive(Serialize, Clone, Default)]
pub struct Search {
    pub query: String,
}
#[function_component(SearchPage)]
pub fn search_page() -> Html {
    let search_value = use_state(Search::default);

    let search_async = {
        let search_value = search_value.clone();
        use_async(async move {
            let query = (*search_value).clone();
            let result = search(&query.query).await;
            console::log!(
                "search page query result",
                json!(result.clone().unwrap()).to_string()
            );

            result
        })
    };

    let onclick = {
        let search_value = search_value.clone();
        let search_async = search_async.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let search = (*search_value).clone();
            let json_str = json!(search);
            console::log!("search page query", json_str.to_string());
            search_async.run();
        })
    };

    let oninput = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut value = (*search_value).clone();
            value.query = input.value();
            search_value.set(value);
        })
    };

    let (data, _count) = if let Some(list_info) = &search_async.data {
        let list = list_info.list.clone();

        (
            list.into_iter()
                .map(SearchListInfoComponent::from)
                .collect::<Vec<SearchListInfoComponent>>(),
            list_info.count,
        )
    } else {
        (vec![], 0)
    };

    // table props
    let table_props = props! {
         TableProps<SearchListInfoComponent> {
            data,
        }
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

            <Table<SearchListInfoComponent> ..table_props>
                <Td name="a" label="A" />
                <Td name="b" label="B" />
                <Td name="c" label="C" centered=true />
            </Table<SearchListInfoComponent>>
        </>
    }
}

impl TableData for SearchListInfoComponent {
    fn get_field_as_html(&self, _field_name: &str) -> Option<yew::Html> {
        None
    }

    // fn get_field_as_value(&self, field_name: &str) -> Option<String> {
    //     match field_name {
    //         "c" => {
    //             let value = serde_json::to_value(self.clone().c).unwrap();
    //             Some(value.as_i64().unwrap().to_string())
    //         }
    //         _ => None,
    //     }
    // }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct SearchListInfoComponent {
    pub a: String,
    pub b: String,
    pub c: i32,
}

impl From<ListInfo> for SearchListInfoComponent {
    fn from(info: ListInfo) -> Self {
        Self {
            a: info.a,
            b: info.b,
            c: info.c,
        }
    }
}
