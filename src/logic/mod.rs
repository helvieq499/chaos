use leptos::*;

pub mod gateway_url;
pub mod socket;
pub mod discord;

pub fn setup(cx: Scope) {
    socket::setup(cx);
}
