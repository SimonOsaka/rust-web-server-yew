use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{api::menu::get_menu, types::menu::MenuItem};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MenuProps {
    #[prop_or("".to_string())]
    pub current: String,
    pub callback: Callback<String>,
}

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let MenuProps { callback, current } = props.clone();

    let get_menu = use_async_with_options(
        async move {
            gloo_console::log!("async request menu::get_menu");
            get_menu().await
        },
        UseAsyncOptions::enable_auto(),
    );

    let third_level = |third_menu_list: Option<Vec<MenuItem>>| {
        if let Some(menu_list) = third_menu_list {
            html! {
                <ul>
                {menu_list.into_iter().map(|menu_third_item| {
                    let onclick = {
                        let menu_third_item = menu_third_item.clone();
                        let callback = callback.clone();
                        Callback::once(move |e: MouseEvent| {
                            e.prevent_default();
                            callback.emit(menu_third_item.name)
                        })
                    };
                    if *current == menu_third_item.clone().name {
                        html! { <li><a class="is-active">{ menu_third_item.clone().label }</a></li> }
                    } else {
                        html! { <li><a onclick={onclick}>{ menu_third_item.clone().label }</a></li> }
                    }
                }).collect::<Html>()}
                </ul>
            }
        } else {
            html! {}
        }
    };

    // second level
    let second_level = |menu_list: Vec<MenuItem>| {
        menu_list.into_iter().map(|menu_second_item| {
            let onclick = {
                let menu_second_item = menu_second_item.clone();
                let callback = callback.clone();
                Callback::once(move |e: MouseEvent| {
                    e.prevent_default();
                    callback.emit(menu_second_item.name)
                })
            };
            if *current == menu_second_item.name {
                html! { <li><a class="is-active">{ menu_second_item.clone().label }</a></li> }
            } else {
                html! { <li><a onclick={onclick}>{ menu_second_item.clone().label }</a>{ third_level(menu_second_item.menu_list) }</li> }
            }
        }).collect::<Html>()
    };

    // first level
    if let Some(menu_wrapper) = &get_menu.data {
        let menu_first_list = &menu_wrapper.menu;
        html! {
            <aside class="menu">
            {menu_first_list.iter().map(|menu_first_item| {
                html! {
                    <>
                        <p class="menu-label">{ menu_first_item.clone().menu_label }</p>
                        <ul class="menu-list">
                            { second_level(menu_first_item.clone().menu_list) }
                        </ul>
                    </>
                }
            }).collect::<Html>()}
            </aside>
        }
    } else {
        html! { "Loading" }
    }
}
