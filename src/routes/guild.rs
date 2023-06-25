use leptos::*;
use leptos_router::*;

#[component(transparent)]
pub fn GuildRoutes(cx: Scope) -> impl IntoView {
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
