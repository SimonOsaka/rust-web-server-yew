use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct SiteInfo {
    pub id: usize,
    pub domain: String,
    pub level: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct SiteInfoWrapper {
    pub list: Vec<SiteInfo>,
}
