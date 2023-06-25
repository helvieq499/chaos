use leptos::*;
use leptos_router::*;

use crate::panels::{AccountPanel, InfoBar};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    crate::bindings::setup(cx);
    crate::logic::setup(cx);

    view! { cx,
        <Router>
            <div id="app_vertical" class="flex">
                <div id="title_bar"></div>
                <div id="app_horizontal" class="flex">
                    <Routes>
                        <Route
                            path="/account"
                            view=move |cx| {
                                view! { cx, <AccountPanel/> }
                            }
                        />
                        <GuildRoutes/>
                    </Routes>
                    <InfoBar/>
                </div>
            </div>
        </Router>
    }
}

#[component(transparent)]
fn GuildRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route
            path="/guilds"
            view=move |_| {
                view! { cx, <Outlet/> }
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
            <Route path="" view=move |_| ()/>
        </Route>
    }
}
