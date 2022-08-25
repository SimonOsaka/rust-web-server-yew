use yew::Classes;

pub mod breadcrumb;
pub mod button;
pub mod calendar;
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
pub mod markdown;
pub mod media_object;
pub mod modal;
pub mod notifications;
pub mod pagination;
pub mod radio;
pub mod section;
pub mod select;
pub mod table;
pub mod tabs;
pub mod tag;
pub mod textarea;

#[derive(PartialEq, Eq, Clone, Debug)]
#[allow(dead_code)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
    Default,
}

impl From<Size> for Classes {
    fn from(size: Size) -> Self {
        let class = match size {
            Size::Small => "is-small",
            Size::Normal => "is-normal",
            Size::Medium => "is-medium",
            Size::Large => "is-large",
            Size::Default => "",
        };
        Self::from(class)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[allow(dead_code)]
pub enum Color {
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    White,
    Light,
    Dark,
    Black,
    Text,
    Ghost,
    Default,
}

impl From<Color> for Classes {
    fn from(color: Color) -> Self {
        let class = match color {
            Color::Primary => "is-primary",
            Color::Danger => "is-danger",
            Color::Info => "is-info",
            Color::Link => "is-link",
            Color::Success => "is-success",
            Color::Warning => "is-warning",
            Color::Default => "",
            Color::White => "is-white",
            Color::Light => "is-light",
            Color::Dark => "is-dark",
            Color::Black => "is-black",
            Color::Text => "is-text",
            Color::Ghost => "is-ghost",
        };
        Self::from(class)
    }
}

pub(crate) fn gen_auto_id() -> String {
    let id = uuid::Uuid::new_v4();
    id.to_string()
}
