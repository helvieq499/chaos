use leptos::*;

mod login;
use login::LoginCheck;

pub mod missing_intents;
use missing_intents::MissingIntents;

#[component]
pub fn InfoBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="panel" id="info_bar">
            <LoginCheck/>
            <MissingIntents/>
        </div>
    }
}
