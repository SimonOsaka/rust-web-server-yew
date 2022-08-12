use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod auth;
pub mod list;
pub mod menu;
pub mod site;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub type EmptyWrapper = HashMap<(), ()>;
