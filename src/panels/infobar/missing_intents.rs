use crate::logic::socket::SocketType;
use leptos::*;

pub struct Enable;

#[component]
pub fn MissingIntents(cx: Scope) -> impl IntoView {
    let signal = use_context::<RwSignal<Enable>>(cx).expect("provided");
    let socket = use_context::<SocketType>(cx).expect("provided");
    let (enabled, set_enabled) = create_signal(cx, false);
    let visibility = move || if enabled() { "" } else { "none" };

    create_effect(cx, move |skip| {
        signal.with(|_| ());

        if skip.is_none() {
            return;
        }

        set_enabled(true);
    });

    create_effect(cx, move |skip| {
        socket.with(|_| ());

        if skip.is_none() {
            return;
        }

        set_enabled(false);
    });

    view! { cx,
        <div class="error" style=("display", visibility)>
            <span class="iconify" data-icon="carbon:error"></span>
            <span>"Intents that you marked when logging in are not enabled."</span>
            <br/>
            <div>
                "Go to " <a href="https://discord.com/developers">"the developers page"</a>
                " and enable the missing intents"
            </div>
        </div>
    }
}

pub fn setup(cx: Scope) {
    provide_context(cx, create_rw_signal(cx, Enable));
}
