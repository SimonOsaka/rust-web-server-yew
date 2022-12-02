use rust_web_server_yew::http::{error::Error, request::request_get};

use crate::types::list::ListInfoWrapper;

/// Get list
pub async fn all(page: usize) -> Result<ListInfoWrapper, Error> {
    request_get::<ListInfoWrapper>(format!("/list/{}", page)).await
}

/// Search list
pub async fn search(query: &str) -> Result<ListInfoWrapper, Error> {
    request_get::<ListInfoWrapper>(format!("/list/{}", query)).await
}
