use crate::{
    components::{
        breadcrumb::{BreadCrumb, BreadCrumbProps},
        menu::{Menu, MenuProps},
        tabs::{Tab, Tabs, TabsProps},
    },
    pages::general::dashboard::Dashboard,
};
use yew::{
    function_component, html, html_nested, props, use_state, virtual_dom::VChild, Callback,
    FunctionComponent,
};

#[function_component(Documentation)]
pub fn documentation() -> Html {
    // breadcrumb
    let bc_props = BreadCrumbProps {
        data: [
            ("Documentation".to_string(), false),
            ("Dashboard".to_string(), true),
        ]
        .to_vec(),
    };

    // tabs
    let current_tab = use_state(|| "".to_string());
    let menu_tab = use_state(|| Vec::<String>::new());
    let current_menu = use_state(|| "".to_string());
    let tabs_toggle_callback = {
        let current_tab = current_tab.clone();
        Callback::from(move |tab| {
            gloo_console::log!(format!("toggle tab = {}", tab));
            current_tab.set(tab);
        })
    };
    let tabs_delete_callback = {
        let current_tab = current_tab.clone();
        let current_menu = current_menu.clone();
        let menu_tab = menu_tab.clone();
        Callback::from(move |tab| {
            gloo_console::log!(format!("delete tab = {}", tab));
            // remove tab menu
            let mut mt = (*menu_tab).clone();
            if mt.contains(&tab) {
                for (i, item) in mt.iter().enumerate() {
                    if *item == tab {
                        mt.remove(i);
                        break;
                    }
                }
                menu_tab.set(mt.clone());
                let s = if let Some(t) = mt.first() {
                    t.to_string()
                } else {
                    "".to_string()
                };
                current_tab.set(s.clone());
                current_menu.set(s);
            }
        })
    };
    let tabs_props = props! {
        TabsProps {
            callback: tabs_toggle_callback,
            callback_delete: tabs_delete_callback
        }
    };

    // menu
    let menu_callback = {
        let current_menu = current_menu.clone();
        let current_tab = current_tab.clone();
        let menu_tab = menu_tab.clone();
        Callback::from(move |menu: String| {
            gloo_console::log!(format!("menu = {}", menu));
            current_menu.set(menu.clone());
            //  add a menu
            let mut mt = (*menu_tab).clone();
            if !mt.contains(&menu) {
                mt.push(menu.clone());
                menu_tab.set(mt);
            }
            current_tab.set(menu);
        })
    };

    let menu_tab_html = {
        let menu_tab = (*menu_tab).clone();
        menu_tab.into_iter().map(|tab| get_menu(tab))
    };

    let menu_props = MenuProps {
        current: (*current_menu).clone(),
        callback: menu_callback,
    };

    html! {
        <>
            <div class="columns">
                <div class="column">
                    <BreadCrumb ..bc_props />
                </div>
            </div>
            <div class="columns">
                <div class="column is-3">
                    <Menu ..menu_props/>
                </div>
                <div class="column is-9">
                    <Tabs current={(*current_tab).clone()}  ..tabs_props>
                        { for menu_tab_html }
                    </Tabs>
                </div>
            </div>
        </>
    }
}

fn get_menu(menu: String) -> VChild<FunctionComponent<crate::components::tabs::tab>> {
    match menu.as_str() {
        "dashboard" => html_nested! {
            <Tab label="Dashboard" name="dashboard">
                <Dashboard />
            </Tab>
        },
        "customers" => html_nested! {
            <Tab label="Customers" name="customers">
                { menu }
            </Tab>
        },
        _ => panic!("No menu found"),
    }
}
