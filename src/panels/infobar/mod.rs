use leptos::*;

mod login;
use login::LoginCheck;

#[component]
pub fn InfoBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="panel" id="info_bar">
            <LoginCheck/>
        </div>
    }
}
