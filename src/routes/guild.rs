use leptos::*;
use leptos_router::*;

use crate::panels::{channel_bar::ChannelBar, guild::list::GuildListPanel, infobar::InfoBar};

#[component(transparent)]
pub fn GuildRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route
            path="/guilds"
            view=move |_| {
                view! { cx,
                    <Outlet/>
                    <InfoBar/>
                }
            }
        >
            <Route
                path=":guild"
                view=move |cx| {
                    view! { cx,
                        <ChannelBar/>
                        <div class="panel" id="message_area"></div>
                    }
                }
            />
            <Route
                path=""
                view=move |cx| {
                    view! { cx, <GuildListPanel/> }
                }
            />
        </Route>
    }
}
