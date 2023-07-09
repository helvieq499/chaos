use std::sync::OnceLock;

use crate::logic::types::Message;
use comrak::{ComrakExtensionOptions, ComrakOptions, ComrakParseOptions, ComrakRenderOptions};
use leptos::*;

pub mod list;

#[component]
pub fn ListedMessage(cx: Scope, message: Message) -> impl IntoView {
    static OPTIONS: OnceLock<ComrakOptions> = OnceLock::new();

    let message_html = comrak::markdown_to_html(
        &message.content,
        OPTIONS.get_or_init(|| ComrakOptions {
            extension: ComrakExtensionOptions {
                strikethrough: true,
                autolink: true,
                tasklist: true,
                ..Default::default()
            },
            parse: ComrakParseOptions {
                smart: false,
                default_info_string: None,
                relaxed_tasklist_matching: false,
            },
            render: ComrakRenderOptions {
                hardbreaks: true,
                ..Default::default()
            },
        }),
    );

    view! { cx, <div inner_html=message_html/> }
}
