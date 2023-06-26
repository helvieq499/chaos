use leptos::{html::Input, *};

#[component]
pub fn AccountTokenLogin(cx: Scope) -> impl IntoView {
    let client = crate::logic::Client::get(cx);
    let credentials = client.credentials;

    let reload = use_context::<RwSignal<super::Reload>>(cx).expect("to be provided");

    let token_elem: NodeRef<Input> = create_node_ref(cx);
    let is_bot_elem: NodeRef<Input> = create_node_ref(cx);

    let login = move |_| {
        let creds = crate::logic::client::credentials::Credentials {
            token: token_elem().expect("element exists").value(),
            is_bot: is_bot_elem().expect("element exists").checked(),
        };

        if let Some(local_storage) = crate::utils::local_storage::get() {
            local_storage
                .set(
                    "credentials",
                    &serde_json::to_string(&creds).expect("parsed successfully"),
                )
                .expect("persisted");
        }

        credentials.set(Some(creds));
        reload.set(super::Reload);
    };

    view! { cx,
        <input node_ref=token_elem placeholder="Token"/>
        <br/>
        <label for="is-bot">"Is Bot"</label>
        <input node_ref=is_bot_elem name="is-bot" type="checkbox"/>
        <br/>
        <button on:click=login>"Login"</button>
    }
}
