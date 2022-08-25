use crate::{error::Error, types::site::SiteInfoWrapper};

use super::request::request_get;

/// Get list
pub async fn all() -> Result<SiteInfoWrapper, Error> {
    request_get::<SiteInfoWrapper>("/sites".to_string()).await
}
