use crate::{
    api::list,
    components::{
        pagination::Pagination,
        pagination::PaginationProps,
        table::{Table, TableProps},
    },
};
use yew::{function_component, html, props, use_effect_with_deps, use_state, Callback};
use yew_hooks::use_async;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let current_page = use_state(|| 1u64);
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
        if !list_info.list.is_empty() {
            let list = list_info.list.clone();
            let data: Vec<Vec<String>> = list
                .into_iter()
                .map(|item| vec![item.a, item.b, item.c])
                .collect();

            // table props
            let table_props = TableProps {
                head: [
                    "Column A".to_string(),
                    "Column B".to_string(),
                    "Column C".to_string(),
                ]
                .to_vec(),
                data,
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
                    <Table ..table_props />
                    <Pagination current={*current_page} ..pagination_props />
                </>
            }
        } else {
            html! {
                { "No data" }
            }
        }
    } else {
        html! {
            { "Loading..." }
        }
    }
}
