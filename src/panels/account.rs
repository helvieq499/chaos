use leptos::{html::*, *};

#[derive(Clone)]
pub struct Reload;

#[component]
pub fn AccountPanel(cx: Scope) -> impl IntoView {
    let reload = use_context::<RwSignal<Reload>>(cx).expect("to be provided");

    let token_elem: NodeRef<Input> = create_node_ref(cx);
    let is_bot_elem: NodeRef<Input> = create_node_ref(cx);

    let login = move |_| {
        if let Some(local_storage) = crate::utils::local_storage::get() {
            local_storage
                .set("token", &token_elem().expect("element exists").value())
                .expect("saved to local storage");

            reload.set(Reload);
        }
    };

    view! { cx,
        <div class="panel" id="account-panel">
            <input node_ref=token_elem placeholder="Token"/>
            <br/>
            <label for="is-bot">"Is Bot"</label>
            <input node_ref=is_bot_elem name="is-bot" type="checkbox"/>
            <br/>
            <button on:click=login>"Login"</button>
        </div>
    }
}

pub fn setup(cx: Scope) {
    provide_context(cx, create_rw_signal(cx, Reload));
}
