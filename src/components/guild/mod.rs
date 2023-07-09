use crate::logic::types::Guild;
use leptos::*;
use leptos_router::*;
use std::rc::Rc;

pub mod list;

#[component]
pub fn ListedGuild(cx: Scope, guild: Rc<Guild>) -> impl IntoView {
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

fn format_image_url(id: &str, icon: &str) -> String {
    format!("url(https://cdn.discordapp.com/icons/{id}/{icon}?size=512)")
}
