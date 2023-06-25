#![warn(clippy::nursery, clippy::pedantic, clippy::unwrap_used)]
#![allow(
    clippy::future_not_send,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value,
    clippy::wildcard_imports
)]

use leptos::*;

mod app;
use app::*;

mod bindings;
mod logic;
mod panels;
mod utils;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(
        wasm_logger::Config::new(
            #[cfg(debug_assertions)]
            log::Level::Trace,
            #[cfg(not(debug_assertions))]
            log::Level::Debug,
        )
        .message_on_new_line(),
    );

    mount_to_body(|cx| view! { cx, <App/> });
}
