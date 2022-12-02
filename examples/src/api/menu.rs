use rust_web_server_yew::http::{error::Error, request::request_get};

use crate::types::menu::MenuWrapper;

pub async fn get_menu() -> Result<MenuWrapper, Error> {
    request_get::<MenuWrapper>("/menu".to_string()).await
}
