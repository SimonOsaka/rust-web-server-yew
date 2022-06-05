pub mod auth;
pub mod list;
pub mod menu;
pub mod request;
pub mod site;

use gloo::storage::{LocalStorage, Storage};
use once_cell::sync::Lazy;
use parking_lot::RwLock;

const TOKEN_KEY: &str = "rust-web-server-yew::token";

static TOKEN: Lazy<RwLock<Option<String>>> = Lazy::new(|| {
    if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
        RwLock::new(Some(token))
    } else {
        RwLock::new(None)
    }
});

/// Set jwt token to local storage.
pub fn set_token(token: Option<String>) {
    if let Some(t) = token.clone() {
        LocalStorage::set(TOKEN_KEY, t).expect("failed to set");
    } else {
        LocalStorage::delete(TOKEN_KEY);
    }
    let mut token_lock = TOKEN.write();
    *token_lock = token;
}

/// Get jwt token from lazy static.
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.clone()
}
