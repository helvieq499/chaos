use leptos::*;

pub mod discord;
pub mod gateway_url;
pub mod socket;

mod client;
pub use client::Client;

pub fn setup(cx: Scope) {
    client::setup(cx);
    socket::setup(cx);
}
