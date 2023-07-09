use leptos::*;
use leptos_router::*;

pub mod info;
pub mod login;

#[component]
pub fn AccountPanel(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="panel" id="account-panel">
            <Outlet/>
        </div>
    }
}
