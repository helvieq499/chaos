use std::rc::Rc;

use leptos::*;

use crate::logic::Client;

#[component]
pub fn TitleBar(cx: Scope) -> impl IntoView {
    let client = use_context::<Rc<Client>>(cx).expect("to be provided");

    let connect = move |_| client.connect.set(true);

    view! { cx,
        <div id="title_bar">
            <button on:click=connect>"Connect"</button>
        </div>
    }
}
