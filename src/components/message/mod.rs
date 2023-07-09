use crate::logic::types::Message;
use leptos::*;

pub mod list;

#[component]
pub fn ListedMessage(cx: Scope, message: Message) -> impl IntoView {
    view! { cx, <div>{message.content}</div> }
}
