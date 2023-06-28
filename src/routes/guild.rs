use leptos::*;
use leptos_router::*;

use crate::panels::{guild::list::GuildListPanel, infobar::InfoBar};

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
                path=":id"
                view=move |cx| {
                    view! { cx,
                        <div class="panel" id="channel_bar"></div>
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
