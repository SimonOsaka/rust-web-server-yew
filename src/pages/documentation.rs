use crate::{
    components::{
        breadcrumb::{BreadCrumb, BreadCrumbProps},
        menu::{Menu, MenuProps},
        tabs::{Tab, Tabs, TabsProps},
    },
    pages::{
        button_page::ButtonPage, chart_page::ChartPage, checkbox_page::CheckboxPage,
        dashboard::Dashboard, delete_page::DeletePage, filters_page::FiltersPage,
        form_page::FormPage, icon_page::FontAwesomeIconPage, image_page::ImagePage,
        input_page::InputPage, loading_page::LoadingPage, modal_page::ModalPage,
        notification_page::NotificationPage, radio_page::RadioPage, search_page::SearchPage,
        select_page::SelectPage, table_page::TablePage, table_tree_page::TableTreePage,
        tag_page::TagPage, textarea_page::TextareaPage,
    },
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
    let menu_tab = use_state(Vec::<String>::new);
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
        menu_tab.into_iter().map(get_menu)
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
        "filters" => html_nested! {
            <Tab label="Filters" name="filters">
                <FiltersPage />
            </Tab>
        },
        "modal" => html_nested! {
            <Tab label="Modal" name="modal">
                <ModalPage />
            </Tab>
        },
        "chart" => html_nested! {
            <Tab label="Chart" name="chart">
                <ChartPage />
            </Tab>
        },
        "notification" => html_nested! {
            <Tab label="Notification" name="notification">
                <NotificationPage />
            </Tab>
        },
        "loading" => html_nested! {
            <Tab label="Loading" name="loading">
                <LoadingPage />
            </Tab>
        },
        "form" => html_nested! {
            <Tab label="Form" name="form">
                <FormPage />
            </Tab>
        },
        "table" => html_nested! {
            <Tab label="Table" name="table">
                <TablePage />
            </Tab>
        },
        "search" => html_nested! {
            <Tab label="Search" name="search">
                <SearchPage />
            </Tab>
        },
        "tree_table" => html_nested! {
            <Tab label="Tree table" name="tree_table">
                <TableTreePage />
            </Tab>
        },
        "tag" => html_nested! {
            <Tab label="Tag" name="tag">
                <TagPage />
            </Tab>
        },
        "button" => html_nested! {
            <Tab label="Button" name="button">
                <ButtonPage />
            </Tab>
        },
        "image" => html_nested! {
            <Tab label="Image" name="image">
                <ImagePage />
            </Tab>
        },
        "icon" => html_nested! {
            <Tab label="Icon" name="icon">
                <FontAwesomeIconPage />
            </Tab>
        },
        "input" => html_nested! {
            <Tab label="Input" name="input">
                <InputPage />
            </Tab>
        },
        "textarea" => html_nested! {
            <Tab label="Textarea" name="textarea">
                <TextareaPage />
            </Tab>
        },
        "select" => html_nested! {
            <Tab label="Select" name="select">
                <SelectPage />
            </Tab>
        },
        "checkbox" => html_nested! {
            <Tab label="Checkbox" name="checkbox">
                <CheckboxPage />
            </Tab>
        },
        "radio" => html_nested! {
            <Tab label="Radio" name="radio">
                <RadioPage />
            </Tab>
        },
        "delete" => html_nested! {
            <Tab label="Delete" name="delete">
                <DeletePage />
            </Tab>
        },
        _ => panic!("No menu found"),
    }
}
