use crate::{
    api::list,
    components::{
        pagination::Pagination,
        pagination::PaginationProps,
        table::{Table, TableData, TableProps, Td},
    },
    types::list::ListInfo,
};
use yew::{function_component, html, props, use_effect_with_deps, use_state, Callback};
use yew_hooks::use_async;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let current_page = use_state(|| 1usize);
    let list_all = {
        let current_page = current_page.clone();
        use_async(async move {
            gloo_console::log!("async request list::all");
            list::all(*current_page).await
        })
    };

    {
        let list_all = list_all.clone();
        use_effect_with_deps(
            move |_| {
                gloo_console::log!("effect list::all");
                list_all.run();

                || ()
            },
            *current_page,
        );
    }

    // pagination props
    let callback = {
        let current_page = current_page.clone();
        Callback::from(move |page| {
            gloo_console::log!(format!("page = {}", page));
            current_page.set(page);
        })
    };

    if let Some(list_info) = &list_all.data {
        // if !list_info.list.is_empty() {
        let data = list_info.list.clone();

        // table props
        let table_props = props! {
             TableProps<ListInfo> {
                data,
            }
        };
        // pagination props
        let pagination_props = props! {
            PaginationProps {
                total: list_info.count,
                callback
            }
        };

        html! {
            <>
                <Table<ListInfo> ..table_props>
                    <Td name="a" label="A" />
                    <Td name="b" label="B" />
                    <Td name="c" label="C" centered=true />
                    <Td name="d" label="D" width=200 centered=true />
                </Table<ListInfo>>
                <Pagination current={*current_page} ..pagination_props />
            </>
        }
        // } else {
        //     html! {
        //         { "No data" }
        //     }
        // }
    } else {
        html! {
            { "Loading..." }
        }
    }
}

impl TableData for ListInfo {
    fn get_field_as_html(&self, field_name: &str) -> Option<yew::Html> {
        match field_name {
            "a" => Some(html! {
                <span class="tag">{self.clone().a}</span>
            }),
            "d" => Some(html! {
                <div class="buttons are-small">
                    <button class="button is-info is-outlined">{"view"}</button>
                    <button class="button is-link is-outlined">{"edit"}</button>
                    <button class="button is-danger is-outlined">{"delete"}</button>
                </div>
            }),
            _ => None,
        }
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
