use leptos::*;
use leptos_router::*;

use crate::components::{
    channel::list::ChannelBar, guild::list::GuildListPanel, infobar::InfoBar,
    message::list::MessagePanel,
};

#[component(transparent)]
pub fn GuildRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route
            path="/guilds"
            view=move |cx| {
                view! { cx,
                    <Outlet/>
                    <InfoBar/>
                }
            }
        >
            <Route
                path=":guild"
                view=move |cx| {
                    view! { cx, <Outlet/> }
                }
            >
                <Route
                    path="/channels"
                    view=move |cx| {
                        view! { cx,
                            <ChannelBar/>
                            <Outlet/>
                        }
                    }
                >
                    <Route
                        path=":channel"
                        view=move |cx| {
                            view! { cx, <MessagePanel/> }
                        }
                    />
                    <Route path="" view=move |_| ()/>
                </Route>
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
