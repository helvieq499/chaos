use leptos::*;
use leptos_router::*;

#[component]
pub fn AccountInfo(cx: Scope) -> impl IntoView {
    let client = crate::logic::Client::get(cx);
    let credentials = client.credentials;

    let logout = move |_| credentials.set(None);

    view! { cx,
        <A href="/account/login">"Login"</A>
        <button on:click=logout>"Logout"</button>
    }
}
