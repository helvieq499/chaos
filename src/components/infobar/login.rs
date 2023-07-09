use leptos::*;
use leptos_router::*;

#[component]
pub fn LoginCheck(cx: Scope) -> impl IntoView {
    let client = crate::logic::Client::get(cx);

    let condition = move || client.credentials.with(Option::is_none);

    view! { cx,
        <Show when=condition fallback=|_| ()>
            <div class="help">
                <span class="iconify" data-icon="carbon:help"></span>
                <span>"You're not " <A href="/account/login/user">"signed in"</A></span>
            </div>
        </Show>
    }
}
