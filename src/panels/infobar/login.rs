use leptos::{html::*, *};
use leptos_router::*;

use crate::panels::account::login::Reload;

#[component]
pub fn LoginCheck(cx: Scope) -> impl IntoView {
    let client = crate::logic::Client::get(cx);
    let notif = create_node_ref::<Div>(cx);
    let reload = use_context::<RwSignal<Reload>>(cx).expect("to be provided");

    create_effect(cx, move |_| {
        reload.get();

        if let Some(elem) = notif.get() {
            let token_exists = client.credentials.with(Option::is_some);
            elem.style("display", if token_exists { "none" } else { "" });
        }
    });

    view! { cx,
        <div node_ref=notif class="help-login-missing" class="help">
            <span class="iconify" data-icon="carbon:help"></span>
            <span>"You're not " <A href="/account/login/user">"signed in"</A></span>
        </div>
    }
}
