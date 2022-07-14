use std::vec;

use crate::components::icon::{FontAwesomeIcon, FontAwesomeIconText};
use yew::prelude::*;

#[function_component(FontAwesomeIconPage)]
pub fn font_awesome_icon_page() -> Html {
    let icons = vec![
        "fa-home",
        "fa-question-circle",
        "fa-exclamation",
        "fa-arrow-down",
        "fa-arrow-up",
        "fa-arrow-left",
        "fa-arrow-right",
        "fa-ban",
        "fa-coffee",
        "fa-calendar",
        "fa-chart-bar",
        "fa-check",
        "fa-clipboard",
        "fa-clipboard-check",
        "fa-clipboard-list",
        "fa-clock",
        "fa-clone",
        "fa-cog",
        "fa-coins",
        "fa-columns",
        "fa-comment",
        "fa-database",
        "fa-desktop",
        "fa-edit",
        "fa-envelope",
        "fa-external-link-alt",
        "fa-filter",
        "fa-flag",
        "fa-folder",
        "fa-font",
        "fa-heart",
        "fa-laptop",
        "fa-link",
        "fa-list",
        "fa-map",
        "fa-mobile",
        "fa-search",
        "fa-server",
        "fa-share",
        "fa-shopping-cart",
        "fa-sort",
        "fa-star",
        "fa-sync",
        "fa-table",
        "fa-tags",
        "fa-tasks",
        "fa-thumbs-up",
        "fa-thumbs-down",
        "fa-tools",
        "fa-trash",
        "fa-tv",
        "fa-user",
    ];

    html! {
        <div class="columns is-multiline">
            {
                icons.iter().map(|icon|{
                    let icon_class = format!("fas {}", icon);
                    html!{
                        <div class="column is-one-quarter">
                            <FontAwesomeIconText text={icon.to_string()}>
                                <FontAwesomeIcon icon={icon_class} />
                            </FontAwesomeIconText>
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
