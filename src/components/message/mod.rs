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

    let migrated = message.author.discriminator == "0";

    let pfp = message.author.avatar.map_or_else(
        || {
            let index = if migrated {
                let id = message.author.id.parse::<u64>().expect("valid");
                (id >> 22) % 6
            } else {
                message.author.discriminator.parse::<u64>().expect("valid") % 5
            };

            format!("https://cdn.discordapp.com/embed/avatars/{index}.png")
        },
        |hash| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png?size=64",
                message.author.id, hash
            )
        },
    );

    let username = if migrated {
        format!("@{}", message.author.username)
    } else {
        format!(
            "{}#{}",
            message.author.username, message.author.discriminator
        )
    };

    view! { cx,
        <div class="message">
            <img class="pfp" src=pfp/>
            <span class="username">{username}</span>
            <div inner_html=message_html></div>
        </div>
    }
}
