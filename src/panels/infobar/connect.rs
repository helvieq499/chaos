use leptos::*;

use crate::logic::Client;

#[component]
pub fn ConnectionCheck(cx: Scope) -> impl IntoView {
    let client = Client::get(cx);
    let connect_signal = client.connect;

    let condition = move || !client.is_connected() && client.credentials.get_untracked().is_some();

    let connect = move |_| connect_signal.set(true);

    view! { cx,
        <Show when=condition fallback=move |_| ()>
            <div class="help">
                <span class="iconify" data-icon="carbon:help"></span>
                <span>
                    "You're not " <a href="#" on:click=connect>
                        "connected"
                    </a>
                </span>
            </div>
        </Show>
    }
}
