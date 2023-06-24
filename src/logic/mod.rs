use leptos::*;

pub mod socket;

pub fn setup(cx: Scope) {
    provide_context(cx, socket::Socket::new(cx));
}
