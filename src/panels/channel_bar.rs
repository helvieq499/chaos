use leptos::*;
use leptos_router::*;

use crate::logic::Client;

#[component]
pub fn ChannelBar(cx: Scope) -> impl IntoView {
    let client = Client::get(cx);

    let param = use_params_map(cx);
    let guild = move || {
        param
            .with(|params| params.get("guild").cloned())
            .and_then(|id| {
                client
                    .guilds
                    .with(|guilds| guilds.get(id.parse().expect("valid id")))
            })
    };

    move || {
        let channels =
            guild().and_then(|guild| guild.extra.as_ref().map(|extra| extra.channels.clone()));

        view! { cx,
            <div class="panel" id="channel_bar">
                {channels
                    .map_or_else(
                        || {
                            view! { cx, "Guild Unavailable" }
                                .into_view(cx)
                        },
                        |channels| {
                            view! { cx,
                                <For
                                    each=move || channels.clone()
                                    key=|channel| { channel.id.parse::<u8>().unwrap_or_default() }
                                    view=|cx, channel| {
                                        view! { cx,
                                            <div class="channel">
                                                <A href=format!("./channel/{}", channel.id)>
                                                    {channel.name.unwrap_or_else(|| String::from("Unnamed channel"))}
                                                </A>
                                            </div>
                                        }
                                    }
                                />
                            }
                                .into_view(cx)
                        },
                    )}
            </div>
        }
    }
}
