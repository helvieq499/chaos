use leptos::*;
use leptos_router::*;

use crate::components::{
    account::{
        info::AccountInfo,
        login::{AccountLogin, AccountTokenLogin, UserLogin},
        AccountPanel,
    },
    infobar::InfoBar,
};

#[component(transparent)]
pub fn AccountRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route
            path="/account"
            view=move |cx| {
                view! { cx,
                    <AccountPanel/>
                    <InfoBar/>
                }
            }
        >
            <Route path="/login" view=AccountLogin>
                <Route path="/token" view=AccountTokenLogin/>
                <Route path="/user" view=UserLogin/>
                <Route path="" view=move |_| ()/>
            </Route>
            <Route path="" view=AccountInfo/>
        </Route>
    }
}
