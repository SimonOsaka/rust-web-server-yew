use crate::{error::Error, types::menu::MenuWrapper};

use super::request::request_get;

pub async fn get_menu() -> Result<MenuWrapper, Error> {
    request_get::<MenuWrapper>("/menu".to_string()).await
}
