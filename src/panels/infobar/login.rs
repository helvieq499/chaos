use leptos::{html::*, *};
use leptos_router::*;

#[component]
pub fn LoginCheck(cx: Scope) -> impl IntoView {
    let notif = create_node_ref::<Div>(cx);
    let reload = use_context::<RwSignal<crate::panels::account::Reload>>(cx).unwrap();

    create_effect(cx, move |_| {
        reload.get();

        if let Some(elem) = notif.get() {
            let token_exists = web_sys::window()
                .unwrap()
                .local_storage()
                .unwrap()
                .unwrap()
                .get("token")
                .unwrap()
                .is_some();

            elem.style("display", if token_exists { "none" } else { "" });
        }
    });

    view! { cx,
        <div node_ref=notif class="help-login-missing" class="help">
            <span class="iconify" data-icon="carbon:help"></span>
            <span>"You're not " <A href="account">"signed in"</A></span>
        </div>
    }
}
