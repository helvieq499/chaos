use leptos::{html::Input, *};

#[component]
pub fn AccountTokenLogin(cx: Scope) -> impl IntoView {
    let client = crate::logic::Client::get(cx);
    let credentials = client.credentials;
    let reload = use_context::<RwSignal<super::Reload>>(cx).expect("to be provided");

    let token_elem: NodeRef<Input> = create_node_ref(cx);
    let (is_bot, set_is_bot) = create_signal(cx, true);
    let (message_intent, set_message_intent) = create_signal(cx, true);

    let login = move |_| {
        let creds = crate::logic::client::credentials::Credentials {
            token: token_elem().expect("element exists").value(),
            is_bot: is_bot(),
            message_intent: message_intent(),
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

    let on_check = |target: WriteSignal<bool>| move |ev| target(event_target_checked(&ev));

    let on_check_is_bot = on_check(set_is_bot);
    let on_check_message_intent = on_check(set_message_intent);

    view! { cx,
        <input node_ref=token_elem placeholder="Token"/>
        <br/>

        <label for="is-bot">"Is Bot"</label>
        <input on:input=on_check_is_bot name="is-bot" type="checkbox" checked=true/>
        <br/>

        <Show when=is_bot fallback=|_| ()>
            <label for="message-intent">"Message Content"</label>
            <input on:input=on_check_message_intent name="message-intent" type="checkbox" checked=false/>
            <label for="message-intent">"(Privileged Intent)"</label>
            <br/>
        </Show>

        <button on:click=login>"Login"</button>

        <Show when=move || !is_bot() fallback=|_| ()>
            <div id="warning-user-account">
                <h2>"User accounts will never be fully supported"</h2>
                <p>"Discord does not document the API for users"</p>
                <p>
                    "EFF: "
                    <a href="https://www.eff.org/deeplinks/2019/10/adversarial-interoperability">
                        "Adversarial Interoperability"
                    </a>
                </p>
            </div>
        </Show>
    }
}
