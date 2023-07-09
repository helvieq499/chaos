use leptos::*;
use leptos_router::*;

use crate::{
    logic::{types::Message, Client},
    utils::request_builder_ext::RequestBuilderExt,
};

#[component]
pub fn MessagePanel(cx: Scope) -> impl IntoView {
    let client = Client::get(cx);

    let params = use_params_map(cx);
    let source = move || params.with(|params| params.get("channel").cloned());

    let messages = create_local_resource(cx, source, move |data| {
        let http = client.http.clone();
        let credentials = client.credentials.get_untracked();
        async move {
            if let Some(creds) = credentials {
                if let Some(channel) = data {
                    if let Ok(response) = http
                        .get(&format!(
                            "https://discord.com/api/v10/channels/{}/messages",
                            &channel
                        ))
                        .auth(&creds)
                        .send()
                        .await
                    {
                        response
                            .text()
                            .await
                            .ok()
                            .and_then(|text| serde_json::from_str::<Vec<Message>>(&text).ok())
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

    let messages = move || {
        messages.with(cx, |messages| {
            messages.as_ref().map(|messages| {
                messages
                    .iter()
                    .rev()
                    .map(|message| {
                        view! { cx,
                            <div>
                                {&message.content}
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
            })
        })
    };

    view! { cx,
        <div id="message_list" class="panel">
            <Suspense fallback=move || view! { cx, <div>"Loading messages"</div> }>
                {messages}
            </Suspense>
        </div>
    }
}
