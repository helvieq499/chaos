use leptos::*;
use leptos_router::*;

use super::ListedChannel;
use crate::{logic::Client, utils::collection_ext::CollectionExt};

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
        let channels = guild().map(|guild| guild.channels.read().expect("unpoisoned").clone());

        view! { cx,
            <div class="panel" id="channel_list">
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
                                    key=|channel| { channel.0 }
                                    view=|cx, channel| {
                                        view! { cx, <ListedChannel channel/> }
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
