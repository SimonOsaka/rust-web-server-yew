use crate::{error::Error, types::list::ListInfoWrapper};

use super::request::request_get;

/// Get list
pub async fn all(page: usize) -> Result<ListInfoWrapper, Error> {
    request_get::<ListInfoWrapper>(format!("/list/{}", page)).await
}
