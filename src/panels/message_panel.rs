use leptos::*;
use leptos_router::*;

use crate::logic::{types::Message, Client};

#[component]
pub fn MessagePanel(cx: Scope) -> impl IntoView {
    let client = Client::get(cx);

    let params = use_params_map(cx);
    let source = move || params.with(|params| params.get("channel").cloned());

    let options = move || {
        client.credentials.get().map(|creds| {
            let headers = js_sys::Object::new();

            js_sys::Reflect::set(
                &headers,
                &js_sys::JsString::from("Authorization"),
                &js_sys::JsString::from(creds.token),
            )
            .expect("reflect set");

            let mut options = web_sys::RequestInit::new();
            options.headers(&headers);
            options
        })
    };

    let messages = create_resource(cx, source, move |data| {
        log::info!("REFETCH");
        let options = options();
        async move {
            if let Some(options) = options {
                if let Some(channel) = data {
                    if let Ok(text) = wasm_bindgen_futures::JsFuture::from(
                        web_sys::window().expect("exists").fetch_with_str_and_init(
                            &format!("https://discord.com/api/v10/channels/{}/messages", &channel),
                            &options,
                        ),
                    )
                    .await
                    .and_then(|resp| web_sys::Request::from(resp).text())
                    {
                        if let Ok(text) = wasm_bindgen_futures::JsFuture::from(text).await {
                            js_sys::JsString::from(text)
                                .as_string()
                                .and_then(|text| serde_json::from_str::<Vec<Message>>(&text).ok())
                                .map(|messages| {
                                    messages
                                        .iter()
                                        .map(|message| message.content.clone())
                                        .collect::<Vec<_>>()
                                })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
    });

    view! { cx,
        <div id="message_panel" class="panel">
            <Suspense fallback=move || view! { cx, <div>"Loading messages"</div> }>
                {messages.read(cx).flatten()}
            </Suspense>
        </div>
    }
}
