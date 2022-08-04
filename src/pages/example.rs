use crate::{
    components::{
        breadcrumb::{BreadCrumb, BreadCrumbs},
        menu::{Menu, MenuProps},
        tabs::{Tab, Tabs, TabsProps},
    },
    pages::{
        button_page::ButtonPage, calendar_page::CalendarPage, chart_page::ChartPage,
        checkbox_page::CheckboxPage, dashboard::Dashboard, delete_page::DeletePage,
        filters_page::FiltersPage, form_page::FormPage, icon_page::FontAwesomeIconPage,
        image_page::ImagePage, input_page::InputPage, loading_page::LoadingPage,
        markdown_page::MarkDownPage, modal_page::ModalPage, notification_page::NotificationPage,
        radio_page::RadioPage, search_page::SearchPage, section_page::SectionPage,
        select_page::SelectPage, table_page::TablePage, table_tree_page::TableTreePage,
        tag_page::TagPage, textarea_page::TextareaPage,
    },
};
use gloo::console;
use linked_hash_map::LinkedHashMap;
use yew::{
    function_component, html, html_nested, props, use_mut_ref, use_state, virtual_dom::VChild,
    Callback, FunctionComponent,
};

#[function_component(Example)]
pub fn example() -> Html {
    // tabs
    let current = use_state(|| "".to_string());

    let tabs_map = use_mut_ref(
        LinkedHashMap::<String, VChild<FunctionComponent<crate::components::tabs::tab>>>::new,
    );

    let tab_click_callback = {
        let current = current.clone();
        Callback::from(move |tab| {
            console::log!(format!("toggle tab = {}", tab));
            current.set(tab);
        })
    };
    let tab_remove_callback = {
        let current = current.clone();
        let tabs_map = tabs_map.clone();
        Callback::from(move |tab: String| {
            console::log!(format!("delete tab = {}", tab));
            // remove tab menu
            let mut map = (*tabs_map.borrow_mut()).clone();
            if map.contains_key(&tab) {
                map.remove(&tab);
                let first = map.front();
                let first_tab = first.map(|(key, _)| key.to_owned());
                *tabs_map.borrow_mut() = map;
                if let Some(t) = first_tab {
                    console::log!(format!("current tab is '{}' after delete {}", t, tab));
                    current.set(t);
                } else {
                    current.set("".into());
                }
            }
        })
    };
    let tabs_props = props! {
        TabsProps {
            current: (*current).clone(),
            callback_click_tab: tab_click_callback,
            callback_remove_tab: tab_remove_callback
        }
    };

    // menu
    let menu_click_callback = {
        let current = current.clone();
        let tabs_map = tabs_map.clone();
        Callback::from(move |menu: String| {
            console::log!(format!("menu = {}", menu));
            current.set(menu.clone());
            // add a tab and container
            let mut map = (*tabs_map.borrow_mut()).clone();
            if !map.contains_key(&menu) {
                map.insert(menu.clone(), get_menu(menu.clone()));
                *tabs_map.borrow_mut() = map;
            }
            current.set(menu);
        })
    };

    let menu_tab_html = {
        let map = (*tabs_map.borrow_mut()).clone();
        map.into_iter().map(|(_, value)| value)
    };

    let menu_props = MenuProps {
        current: (*current).clone(),
        callback: menu_click_callback,
    };

    html! {
        <>
            <div class="columns">
                <div class="column">
                    <BreadCrumbs>
                        <BreadCrumb title={"Example"} link={Some("#")} />
                        <BreadCrumb title={"Dashboard"} />
                    </BreadCrumbs>
                </div>
            </div>
            <div class="columns">
                <div class="column is-3">
                    <Menu ..menu_props/>
                </div>
                <div class="column is-9">
                    <Tabs ..tabs_props>
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
            <Tab key="dashboard" label="Dashboard" name="dashboard">
                <Dashboard />
            </Tab>
        },
        "filters" => html_nested! {
            <Tab key="filters" label="Filters" name="filters">
                <FiltersPage />
            </Tab>
        },
        "modal" => html_nested! {
            <Tab key="modal" label="Modal" name="modal">
                <ModalPage />
            </Tab>
        },
        "chart" => html_nested! {
            <Tab key="chart" label="Chart" name="chart">
                <ChartPage />
            </Tab>
        },
        "notification" => html_nested! {
            <Tab key="notification" label="Notification" name="notification">
                <NotificationPage />
            </Tab>
        },
        "loading" => html_nested! {
            <Tab key="loading" label="Loading" name="loading">
                <LoadingPage />
            </Tab>
        },
        "form" => html_nested! {
            <Tab key="form" label="Form" name="form">
                <FormPage />
            </Tab>
        },
        "table" => html_nested! {
            <Tab key="table" label="Table" name="table">
                <TablePage />
            </Tab>
        },
        "search" => html_nested! {
            <Tab key="search" label="Search" name="search">
                <SearchPage />
            </Tab>
        },
        "tree_table" => html_nested! {
            <Tab key="tree_table" label="Tree table" name="tree_table">
                <TableTreePage />
            </Tab>
        },
        "tag" => html_nested! {
            <Tab key="tag" label="Tag" name="tag">
                <TagPage />
            </Tab>
        },
        "button" => html_nested! {
            <Tab key="button" label="Button" name="button">
                <ButtonPage />
            </Tab>
        },
        "image" => html_nested! {
            <Tab key="image" label="Image" name="image">
                <ImagePage />
            </Tab>
        },
        "icon" => html_nested! {
            <Tab key="icon" label="Icon" name="icon">
                <FontAwesomeIconPage />
            </Tab>
        },
        "input" => html_nested! {
            <Tab key="input" label="Input" name="input">
                <InputPage />
            </Tab>
        },
        "textarea" => html_nested! {
            <Tab key="textarea" label="Textarea" name="textarea">
                <TextareaPage />
            </Tab>
        },
        "select" => html_nested! {
            <Tab key="select" label="Select" name="select">
                <SelectPage />
            </Tab>
        },
        "checkbox" => html_nested! {
            <Tab key="checkbox" label="Checkbox" name="checkbox">
                <CheckboxPage />
            </Tab>
        },
        "radio" => html_nested! {
            <Tab key="radio" label="Radio" name="radio">
                <RadioPage />
            </Tab>
        },
        "delete" => html_nested! {
            <Tab key="delete" label="Delete" name="delete">
                <DeletePage />
            </Tab>
        },
        "section" => html_nested! {
            <Tab key="section" label="Section" name="section">
                <SectionPage />
            </Tab>
        },
        "calendar" => html_nested! {
            <Tab key="calendar" label="Calendar" name="calendar">
                <CalendarPage />
            </Tab>
        },
        "markdown" => html_nested! {
            <Tab key="markdown" label="Markdown" name="markdown">
                <MarkDownPage />
            </Tab>
        },
        _ => panic!("No menu found"),
    }
}
