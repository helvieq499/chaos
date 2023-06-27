use leptos::*;
use leptos_router::*;

use crate::panels::title_bar::TitleBar;
use crate::routes::{AccountRoutes, GuildRoutes};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    crate::bindings::setup(cx);
    crate::logic::setup(cx);

    view! { cx,
        <Router>
            <div id="app_vertical" class="flex">
                <TitleBar/>
                <div id="app_horizontal" class="flex">
                    <Routes>
                        <AccountRoutes/>
                        <GuildRoutes/>
                    </Routes>
                </div>
            </div>
        </Router>
    }
}
