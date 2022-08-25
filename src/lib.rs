pub mod bridge;
pub mod components;

/// debug mode will output log message
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log {
    ($($arg:expr),+) => {
        gloo::console::log!($($arg),+);
    }
}

/// release mode won't output log message
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log {
    ($($arg:expr),+) => {};
}
