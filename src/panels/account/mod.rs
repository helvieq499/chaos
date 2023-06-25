use leptos::*;
use leptos_router::*;

mod token_login;
pub use token_login::AccountTokenLogin;

#[derive(Clone)]
pub struct Reload;

pub fn setup(cx: Scope) {
    provide_context(cx, create_rw_signal(cx, Reload));
}

#[component]
pub fn AccountPanel(cx: Scope) -> impl IntoView {
    let client = crate::logic::Client::get(cx);
    let credentials = client.credentials;

    let reload = use_context::<RwSignal<Reload>>(cx).expect("to be provided");

    let logout = move |_| {
        if let Some(local_storage) = crate::utils::local_storage::get() {
            local_storage.remove_item("credentials").expect("persisted");
        }

        credentials.set(None);
        reload.set(Reload);
    };

    view! { cx,
        <div class="panel" id="account-panel">
            <div class="flex" id="account-panel-login-options">
                <A href="/account/login/user">"User Login"</A>
                <A href="/account/login/token">"Token Login"</A>
            </div>
            <Outlet/>
            <button on:click=logout>"Logout"</button>
        </div>
    }
}
