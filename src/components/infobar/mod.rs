use leptos::*;

mod ctd;
pub use ctd::CTDCheck;

mod connect;
pub use connect::ConnectionCheck;

mod login;
use login::LoginCheck;

pub mod missing_intents;
use missing_intents::MissingIntents;

#[component]
pub fn InfoBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="panel" id="info_bar">
            <CTDCheck/>
            <LoginCheck/>
            <ConnectionCheck/>
            <MissingIntents/>
        </div>
    }
}
