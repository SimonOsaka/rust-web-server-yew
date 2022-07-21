use crate::{
    api::list,
    bridge::loading_agent::{LoadingAgent, LoadingInput},
    components::{
        loading::LoadingProps,
        pagination::Pagination,
        pagination::PaginationProps,
        table::{Table, TableData, TableProps, Td},
    },
    types::list::ListInfo,
};
use gloo::timers::future::TimeoutFuture;
use serde::{Deserialize, Serialize};
use yew::{function_component, html, props, use_effect_with_deps, use_state, Callback, Properties};
use yew_agent::use_bridge;
use yew_hooks::use_async;

#[function_component(TablePage)]
pub fn table_page() -> Html {
    let loading_agent = use_bridge::<LoadingAgent, _>(|_| {});

    let current_page = use_state(|| 1usize);
    // let request_approve = use_state(|| true);

    let list_all = {
        let current_page = current_page.clone();
        // let request_approve = request_approve.clone();
        use_async(async move {
            gloo_console::log!("async request list::all");
            // open loading
            let loading_props = LoadingProps::builder().loading(true).build();
            loading_agent.send(LoadingInput::Input(loading_props));
            TimeoutFuture::new(3000).await;

            let result = list::all(*current_page).await;

            // close loading
            let loading_props = LoadingProps::builder().loading(false).build();
            loading_agent.send(LoadingInput::Input(loading_props));

            // request_approve.set(false);

            result
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
            // (*current_page, *request_approve),
            *current_page,
        );
    }

    // pagination props
    let pagination_click_callback = {
        let current_page = current_page.clone();
        // let request_approve = request_approve.clone();
        Callback::from(move |page| {
            gloo_console::log!(format!("page = {}", page));
            current_page.set(page);
            // request_approve.set(true);
        })
    };

    let (data, count) = if let Some(list_info) = &list_all.data {
        let list = list_info.list.clone();

        (
            list.into_iter()
                .map(ListInfoComponent::from)
                .collect::<Vec<ListInfoComponent>>(),
            list_info.count,
        )
    } else {
        (vec![], 0)
    };

    // table props
    let table_props = props! {
         TableProps<ListInfoComponent> {
            data,
        }
    };
    // pagination props
    let pagination_props = props! {
        PaginationProps {
            total: count,
            callback: pagination_click_callback
        }
    };

    html! {
        <>
            <Table<ListInfoComponent> ..table_props>
                <Td name="a" label="A" />
                <Td name="b" label="B" />
                <Td name="c" label="C" centered=true />
                <Td name="d" label="D" width=200 centered=true />
            </Table<ListInfoComponent>>
            <Pagination current={*current_page} ..pagination_props />
        </>
    }
}

impl TableData for ListInfoComponent {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ListInfoComponent {
    pub a: String,
    pub b: String,
    pub c: i32,
}

impl From<ListInfo> for ListInfoComponent {
    fn from(info: ListInfo) -> Self {
        Self {
            a: info.a,
            b: info.b,
            c: info.c,
        }
    }
}
