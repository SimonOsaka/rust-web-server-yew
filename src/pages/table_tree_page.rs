use crate::{
    api::site,
    components::table::{Table, TableData, TableProps, Td},
    types::site::SiteInfo,
};
use gloo::console;
use serde::{Deserialize, Serialize};
use web_sys::MouseEvent;
use yew::{function_component, html, props, Callback};
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[function_component(TableTreePage)]
pub fn table_tree_page() -> Html {
    let site_all = {
        use_async_with_options(
            async move {
                console::log!("async request site::all");

                site::all().await
            },
            UseAsyncOptions::enable_auto(),
        )
    };

    let data = if let Some(site_info) = &site_all.data {
        let list = site_info.list.clone();

        list.into_iter()
            .map(SiteInfoComponent::from)
            .collect::<Vec<SiteInfoComponent>>()
    } else {
        vec![]
    };

    // table props
    let table_props = props! {
         TableProps<SiteInfoComponent> {
            data,
        }
    };

    html! {
        <>
            <Table<SiteInfoComponent> ..table_props>
                <Td name="id" label="Id" />
                <Td name="domain" label="Domain" />
                <Td name="sub_domain" label="Sub domain" />
                <Td name="operation" label="Operation" width=200 centered=true />
            </Table<SiteInfoComponent>>
        </>
    }
}

impl TableData for SiteInfoComponent {
    fn get_field_as_html(&self, field_name: &str) -> Option<yew::Html> {
        match field_name {
            "domain" => match self.level {
                0 => Some(html! {
                    {self.clone().domain}
                }),
                _ => Some(html! {}),
            },
            "sub_domain" => match self.level {
                1 => Some(html! {
                    {self.clone().domain}
                }),
                _ => Some(html! {}),
            },
            "operation" => {
                let onclick = {
                    let id = self.id;
                    Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        console::log!("view", id);
                    })
                };
                Some(html! {
                    <div class="buttons are-small">
                        <button class="button is-info is-outlined" {onclick}>{"view"}</button>
                        <button class="button is-link is-outlined">{"edit"}</button>
                        <button class="button is-danger is-outlined">{"delete"}</button>
                    </div>
                })
            }
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct SiteInfoComponent {
    pub id: usize,
    pub domain: String,
    pub level: u8,
}

impl From<SiteInfo> for SiteInfoComponent {
    fn from(info: SiteInfo) -> Self {
        Self {
            id: info.id,
            domain: info.domain,
            level: info.level,
        }
    }
}
