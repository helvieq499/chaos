use crate::logic::types::Channel;
use leptos::*;
use leptos_router::*;
use std::rc::Rc;

pub mod list;

#[component]
pub fn ListedChannel(cx: Scope, channel: (u64, Rc<Channel>)) -> impl IntoView {
    view! { cx,
        <div class="channel">
            <A href=format!("./{}", channel.0)>
                {channel.1.name.as_ref().cloned().unwrap_or_else(|| String::from("Unnamed channel"))}
            </A>
        </div>
    }
}
