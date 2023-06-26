use leptos::*;
use leptos_router::*;

use crate::panels::account::{
    info::AccountInfo,
    login::{AccountLogin, AccountTokenLogin},
    AccountPanel,
};

#[component(transparent)]
pub fn AccountRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route
            path="/account"
            view=move |cx| {
                view! { cx, <AccountPanel/> }
            }
        >
            <Route
                path="/login"
                view=move |cx| {
                    view! { cx, <AccountLogin/> }
                }
            >
                <Route
                    path="/token"
                    view=move |cx| {
                        view! { cx, <AccountTokenLogin/> }
                    }
                />
                <Route path="/user" view=move |_| ()/>
                <Route path="" view=move |_| ()/>
            </Route>
            <Route
                path=""
                view=move |cx| {
                    view! { cx, <AccountInfo/> }
                }
            />
        </Route>
    }
}
