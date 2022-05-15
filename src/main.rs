mod api;
mod app;
mod components;
mod error;
mod hooks;
mod pages;
mod types;

use app::App;

fn main() {
    yew::start_app::<App>();
}
