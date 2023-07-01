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
                    view! { cx, <ChannelBar/> }
                }
            >
                <Route path="/channels" view=move |_| ()/>
                <Route path="" view=move |_| ()/>
            </Route>

            <Route
                path=""
                view=move |cx| {
                    view! { cx, <GuildListPanel/> }
                }
            />
        </Route>
    }
}
