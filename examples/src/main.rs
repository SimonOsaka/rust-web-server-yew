mod api;
mod app;
mod hooks;
mod pages;
mod types;

use api::mock;
use app::App;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    mock();
    yew::start_app::<App>();
}
