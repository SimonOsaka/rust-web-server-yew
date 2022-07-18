use std::{borrow::Cow, fmt::Display};

pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod chart;
pub mod checkbox;
pub mod delete;
pub mod filter;
pub mod footer;
pub mod hero;
pub mod icon;
pub mod image;
pub mod input;
pub mod loading;
pub mod media_object;
pub mod menu;
pub mod modal;
pub mod navbar;
pub mod notifications;
pub mod pagination;
pub mod radio;
pub mod section;
pub mod select;
pub mod table;
pub mod tabs;
pub mod tag;
pub mod textarea;
pub mod user_context_provider;

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
    Default,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            Size::Small => "is-small",
            Size::Normal => "is-normal",
            Size::Medium => "is-medium",
            Size::Large => "is-large",
            Size::Default => "",
        };
        f.write_str(class)
    }
}
