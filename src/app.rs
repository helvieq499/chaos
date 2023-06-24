use leptos::*;

use crate::components::infobar::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx, 
        <div id="app_vertical" class="flex">
            <div id="title_bar"/>
            <div id="app_horizontal" class="flex">
                <div id="channel_bar"/>
                <div id="message_area"/>
                <InfoBar/>
            </div>
        </div>
    }
}
