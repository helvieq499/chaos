use std::rc::Rc;

use leptos::*;
use leptos_router::*;

use crate::logic::{types::Guild, Client};

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
                                view! { cx,
                                    <A href=format!("/guilds/{}/channels", guild.id)>
                                        {
                                            view! { cx,
                                                <div style=(
                                                    "background-image",
                                                    {
                                                        guild
                                                            .info
                                                            .as_ref()
                                                            .and_then(|info| info.icon.as_ref())
                                                            .map_or(String::new(), |icon| { format_image_url(&guild.id, icon) })
                                                    },
                                                )>
                                                    <div>
                                                        {guild.info.as_ref().map_or_else(|| String::from("Unavailable"), |info| info.name.clone())}
                                                    </div>
                                                    <div class="guild_id">{&guild.id}</div>
                                                </div>
                                            }
                                        }
                                    </A>
                                }
                            }
                        />
                    </div>
                </Show>
            </div>
        }
    }
}

fn format_image_url(id: &str, icon: &str) -> String {
    format!("url(https://cdn.discordapp.com/icons/{id}/{icon}?size=512)")
}
