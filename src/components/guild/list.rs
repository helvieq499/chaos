use std::rc::Rc;

use super::ListedGuild;
use crate::logic::{types::Guild, Client};
use leptos::*;

#[component]
pub fn GuildListPanel(cx: Scope) -> impl IntoView {
    let client = Client::get(cx);
    let guilds = client.guilds;

    let guild_list = move || {
        guilds.with(|guilds| {
            guilds
                .read()
                .expect("unpoisoned")
                .values()
                .map(std::clone::Clone::clone)
                .collect::<Vec<Rc<Guild>>>()
        })
    };

    move || {
        let guilds = guild_list();

        view! { cx,
            <div id="guild_list" class="panel">
                <Show
                    when=move || !guilds.is_empty()
                    fallback=move |cx| {
                        view! { cx, <div>"No Guilds"</div> }
                    }
                >
                    <div id="guild_list_container">
                        <For
                            each=guild_list
                            key=|guild| guild.key()
                            view=move |cx, guild| {
                                view! { cx, <ListedGuild guild/> }
                            }
                        />
                    </div>
                </Show>
            </div>
        }
    }
}
