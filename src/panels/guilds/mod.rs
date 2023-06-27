use std::rc::Rc;

use leptos::*;
use leptos_router::*;

use crate::logic::{client::guild::Guild, Client};

#[component]
pub fn GuildList(cx: Scope) -> impl IntoView {
    let client = Client::get(cx);
    let guilds = client.guilds;

    let guild_list = move || {
        guilds.with(|guilds| {
            guilds
                .read()
                .expect("unpoisoned")
                .values()
                .map(|guild| guild.clone())
                .collect::<Vec<Rc<Guild>>>()
        })
    };

    move || {
        let guilds = guild_list();

        view! { cx,
            <div id="guild_list" class="panel">
                <Show
                    when=move || !guilds.is_empty()
                    fallback=move |cx| view! { cx,
                        <div>"No Guilds"</div>
                    }
                >
                    <div id="guild_list_container">
                        <For
                            each=guild_list
                            key=|guild| guild.key()
                            view=move |cx, guild| view! { cx,
                                <A href={format!("/guilds/{}", guild.key_str())}>
                                    {
                                        match guild.as_ref() {
                                            Guild::Unavailable(guild) => {
                                                view! { cx,
                                                    <div>
                                                        <div>"Unavailable Guild"</div>
                                                        <span class="iconify" data-icon="carbon:error-filled"></span>
                                                        <div class="guild_id">{&guild.id}</div>
                                                    </div>
                                                }
                                            }
                                            Guild::Available(guild) => {
                                                view! { cx,
                                                    <div style=(
                                                        "background-image",
                                                        {
                                                            guild
                                                                .icon
                                                                .as_ref()
                                                                .map_or("".to_string(), |icon| { format_image_url(&guild.id, icon) })
                                                        },
                                                    )>
                                                        <div>{&guild.name}</div>
                                                        <div class="guild_id">{&guild.id}</div>
                                                    </div>
                                                }
                                            }
                                        }
                                    }
                                </A>
                            }
                        />
                    </div>
                </Show>
            </div>
        }
    }

    // view! { cx,
    //     <For
    //         each=guild_list
    //         key=|guild| guild.key()
    //         view=move |cx, guild| view! { cx,

    //         }
    //     />
    // }

    // let fallback = move |cx| {
    //     view! { cx, <div>"No Guilds"</div> }
    // };

    // let guilds_view = move || {
    //     guilds.with(|guilds| {
    //         guilds
    //             .read()
    //             .expect("not poisoned")
    //             .values()
    //             .map(move |guild| match guild {
    //                 Guild::Unavailable(guild) => {
    //                     view! { cx,
    //                         <div>
    //                             <div>"Unavailable Guild"</div>
    //                             <span class="iconify" data-icon="carbon:error-filled"></span>
    //                             <div>{&guild.id}</div>
    //                         </div>
    //                     }
    //                 }
    //                 Guild::Available(guild) => {
    //                     view! { cx,
    //                         <div style=(
    //                             "background-image",
    //                             {
    //                                 guild
    //                                     .icon
    //                                     .as_ref()
    //                                     .map_or("".to_string(), |icon| { format_image_url(&guild.id, icon) })
    //                             },
    //                         )>
    //                             <div>{&guild.name}</div>
    //                             <div>{&guild.id}</div>
    //                         </div>
    //                     }
    //                 }
    //             })
    //             .collect::<Vec<_>>()
    //     })
    // };

    // view! { cx,
    //     <div id="guild_list" class="panel">
    //         <Show
    //             when=move || guilds.with(|guilds| guilds.read().expect("not poisoned").len() > 0)
    //             fallback=fallback
    //         >
    //             <div id="guild_list_container">{guilds_view}</div>
    //         </Show>
    //     </div>
    // }
}

fn format_image_url(id: &str, icon: &str) -> String {
    format!("url(https://cdn.discordapp.com/icons/{id}/{icon}?size=512)")
}
