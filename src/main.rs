use leptos::*;

mod app;
use app::*;

mod bindings;
mod logic;
mod panels;
mod utils;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default().message_on_new_line());

    mount_to_body(|cx| view! { cx, <App/> })
}
