use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod auth;
pub mod list;
pub mod menu;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub type EmptyWrapper = HashMap<(), ()>;
