use leptos::*;
use leptos_router::*;

mod token_login;
pub use token_login::AccountTokenLogin;

#[component]
pub fn AccountLogin(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex" id="account-panel-login-options">
            <A href="/account/login/user">"User Login"</A>
            <A href="/account/login/token">"Token Login"</A>
        </div>
        <Outlet/>
    }
}
