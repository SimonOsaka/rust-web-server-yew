use rust_web_server_yew::http::{error::Error, request::request_get};

use crate::types::site::SiteInfoWrapper;

/// Get list
pub async fn all() -> Result<SiteInfoWrapper, Error> {
    request_get::<SiteInfoWrapper>("/sites".to_string()).await
}
