use leptos::*;
use leptos_router::*;

#[component]
pub fn TitleBar(cx: Scope) -> impl IntoView {
    let client = crate::logic::Client::get(cx);

    let connect = move |_| client.connect.set(true);

    view! { cx,
        <div id="title_bar">
            <A href="/account">"Account"</A>
            <A href="/guilds">"Guilds"</A>
            <button on:click=connect>"Connect"</button>
        </div>
    }
}
