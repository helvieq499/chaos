use leptos::*;
use leptos_router::*;

#[component]
pub fn AccountInfo(cx: Scope) -> impl IntoView {
    let client = crate::logic::Client::get(cx);
    let credentials = client.credentials;

    let logout = move |_| {
        if let Some(local_storage) = crate::utils::local_storage::get() {
            local_storage.remove_item("credentials").expect("persisted");
        }

        credentials.set(None);
    };

    let connect = move |_| client.connect.set(true);

    view! { cx,
        <A href="/account/login">"Login"</A>
        <button on:click=logout>"Logout"</button>
        <br/>
        <button on:click=connect>"Connect"</button>
    }
}
