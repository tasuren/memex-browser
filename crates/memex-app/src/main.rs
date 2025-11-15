pub mod foundation;
pub mod platform_impl;
pub mod system;
pub mod ui;
mod window;

pub use window::*;

#[cfg(debug_assertions)]
pub const APP_IDENTIFIER: &str = "jp.tasuren.memex-poc-dev";
#[cfg(not(debug_assertions))]
pub const APP_IDENTIFIER: &str = "jp.tasuren.memex-poc";

fn main() {
    system::boot();
}
