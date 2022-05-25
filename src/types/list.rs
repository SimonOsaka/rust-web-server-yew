use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ListInfo {
    pub a: String,
    pub b: String,
    pub c: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ListInfoWrapper {
    pub list: Vec<ListInfo>,
    pub count: usize,
}
