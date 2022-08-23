use crate::api::list;
use crate::bridge::loading_agent::{LoadingAgent, LoadingInput};
use crate::components::{
    checkbox::Checkbox,
    loading::LoadingProps,
    pagination::{Pagination, PaginationProps},
    table::{Table, Td, Th},
};
use gloo::{console, timers::future::TimeoutFuture};
use yew::{
    function_component, html, props, use_mut_ref, Callback, Html,
    Properties,
};
use yew_agent::use_bridge;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[function_component(TablePage)]
pub fn table_page() -> Html {
    let loading_agent = use_bridge::<LoadingAgent, _>(|_| {});

    let current_page = use_mut_ref(|| 1usize);

    let list_all = {
        let current_page = current_page.clone();

        use_async_with_options(async move {
            console::log!("async request list::all");
            // open loading
            let loading_props = LoadingProps::builder().loading(true).build();
            loading_agent.send(LoadingInput::Input(loading_props));
            TimeoutFuture::new(500).await;

            let query = *current_page.borrow();
            let result = list::all(query).await;

            // close loading
            let loading_props = LoadingProps::builder().loading(false).build();
            loading_agent.send(LoadingInput::Input(loading_props));


            result
        }, UseAsyncOptions::enable_auto())
    };

    let state_checkbox = use_mut_ref(Vec::<i32>::default);

    // pagination props
    let pagination_click_callback = {
        let current_page = current_page.clone();
        let list_all = list_all.clone();
        let state_checkbox = state_checkbox.clone();

        Callback::from(move |page| {
            console::log!(format!("page = {}", page));
            *current_page.borrow_mut() = page;

            *state_checkbox.borrow_mut() = Vec::<i32>::default();

            console::log!("page list all run()");
            list_all.run();
        })
    };

    let (data, count) = if let Some(list_info) = &list_all.data {
        let list = list_info.list.clone();

        (list, list_info.count)
    } else {
        (vec![], 0)
    };

    // table props
    let thead_html = {
        html! {
            <tr>
                <Th></Th>
                <Th>{"A"}</Th>
                <Th>{"B"}</Th>
                <Th>{"C"}</Th>
                <Th centered=true width=200>{"D"}</Th>
            </tr>
        }
    };

    

    let table_html = {
        data.iter()
            .map(|item| {
                let callback_checkbox = {
                    let state_checkbox = state_checkbox.clone();
                    Callback::from(move |(checked, value):(bool,String)| {
                        gloo::console::log!(format!("checked: {}, value: {}", checked, value));
                        
                        let c = value.parse::<i32>().unwrap();
                        let mut state_clone = (*state_checkbox).clone();
                        let vec = state_clone.get_mut();
                        if vec.contains(&c) {
                            vec.retain(|x| *x!=c);
                        } else {
                            vec.push(c);
                        }
                        *state_checkbox.borrow_mut() = vec.to_vec();

                        gloo::console::log!(format!("{:?}",*state_checkbox.borrow()));

                    })
                };

                html! {
                    <tr>
                        <Td>
                            <Checkbox value={item.c.to_string()} callback={callback_checkbox}></Checkbox>
                        </Td>
                        <Td>
                            <span class="tag">{item.a.clone()}</span>
                        </Td>
                        <Td>
                            {item.b.clone()}
                        </Td>
                        <Td centered=true>
                            {item.c}
                        </Td>
                        <Td centered=true>
                            <div class="buttons are-small">
                                <button class="button is-info is-outlined">{"view"}</button>
                                <button class="button is-link is-outlined">{"edit"}</button>
                                <button class="button is-danger is-outlined">{"delete"}</button>
                            </div>
                        </Td>
                    </tr>
                }
            })
            .collect::<Html>()
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
            <Table>
                { thead_html }
                {
                    if list_all.loading {
                        html! {
                            <tr>
                                <Td colspan=5 centered=true>{ "Loading..." }</Td>
                            </tr>
                        }
                    } else if count == 0 {
                        html! {
                            <tr>
                                <Td colspan=5 centered=true>{ "No data" }</Td>
                            </tr>
                        }
                    } else {
                        table_html
                    }
                }
            </Table>
            <Pagination current={*current_page.borrow()} ..pagination_props />
        </>
    }
}
