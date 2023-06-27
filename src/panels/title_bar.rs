use leptos::*;
use leptos_router::*;

#[component]
pub fn TitleBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="title_bar">
            <A href="/account">"Account"</A>
            <A href="/guilds">"Guilds"</A>
        </div>
    }
}
