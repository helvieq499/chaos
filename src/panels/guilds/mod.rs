use leptos::*;

use crate::logic::{client::guild::Guild, Client};

#[component]
pub fn GuildList(cx: Scope) -> impl IntoView {
    let client = Client::get(cx);
    let guilds = client.guilds;

    let fallback = move |cx| {
        view! { cx, <h1>"No Guilds"</h1> }
    };

    view! { cx,
        <div id="guild_list" class="panel">
            <Show
                when=move || guilds.with(|guilds| guilds.read().expect("not poisoned").len() > 0)
                fallback=fallback
            >
                {guilds
                    .with(|guilds| {
                        guilds
                            .read()
                            .expect("not poisoned")
                            .values()
                            .map(move |guild| {
                                match guild {
                                    Guild::Unavailable(guild) => {
                                        view! { cx, <div>"Unavailable Guild (" {&guild.id} ")"</div> }
                                    }
                                    Guild::Available(guild) => {
                                        view! { cx, <div>{&guild.name} " (" {&guild.id} ")"</div> }
                                    }
                                }
                            })
                            .collect::<Vec<_>>()
                    })}
            </Show>
        </div>
    }
}
