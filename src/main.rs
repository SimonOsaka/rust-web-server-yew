mod api;
mod app;
mod bridge;
mod components;
mod error;
mod hooks;
mod pages;
mod types;

use app::App;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    yew::start_app::<App>();
}
