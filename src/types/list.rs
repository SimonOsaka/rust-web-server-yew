use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ListInfo {
    pub a: String,
    pub b: String,
    pub c: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ListInfoWrapper {
    pub list: Vec<ListInfo>,
    pub count: u64,
}
