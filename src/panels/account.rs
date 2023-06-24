use leptos::{html::*, *};

#[derive(Clone)]
pub struct Reload;

#[component]
pub fn AccountPanel(cx: Scope) -> impl IntoView {
    let reload = use_context::<RwSignal<Reload>>(cx).unwrap();
    let token_elem: NodeRef<Input> = create_node_ref(cx);
    let login = move |_| {
        web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .set("token", &token_elem().unwrap().value())
            .unwrap();
        reload.set(Reload);
    };

    view! { cx,
        <div class="panel" id="account-panel">
            <input node_ref=token_elem placeholder="Token"/>
            <button on:click=login>"Login"</button>
        </div>
    }
}

pub fn setup(cx: Scope) {
    provide_context(cx, create_rw_signal(cx, Reload));
}
