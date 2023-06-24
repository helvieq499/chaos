use leptos::*;

mod app;
use app::*;

mod components;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    mount_to_body(|cx| view! { cx, <App/> })
}
