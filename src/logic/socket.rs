use leptos::*;
use wasm_sockets::{EventClient, Message};

use super::discord::RecvEvent;

pub fn setup(cx: Scope) {
    let socket = create_rw_signal::<Option<wasm_sockets::EventClient>>(cx, None);
    provide_context(cx, socket);

    let gateway_url = super::gateway_url::resource(cx);

    create_effect(cx, move |_| {
        gateway_url.with(cx, |res| {
            if let Some((from_local, url)) = res {
                log::debug!(
                    "Gateway URL: {}\nFetched from {} source",
                    url,
                    if *from_local { "local" } else { "remote" }
                );

                if let Ok(mut client) = EventClient::new(url) {
                    client.set_on_connection(Some(Box::new(on_connection)));
                    client.set_on_message(Some(Box::new(on_message)));
                    client.set_on_close(Some(Box::new(on_close)));
                    client.set_on_error(Some(Box::new(on_error)));
                    socket.set(Some(client));
                }
            } else {
                log::error!("Failed to fetch gateway URL");
            }
        })
    });
}

fn on_connection(_this: &EventClient) {
    log::info!("Connected to socket");
}

fn on_message(_this: &EventClient, msg: Message) {
    if let Message::Text(ref text) = msg {
        if let Ok(json) = json::parse(&text) {
            if let Ok(event) = RecvEvent::try_from(json) {
                log::debug!("{:?}", event);
            }
        }
    } else {
        log::warn!("Socket received an unknown binary message");
    }
}

fn on_close(close: web_sys::CloseEvent) {
    log::warn!("Socket closed gracefully\n{:?}", close);
}

fn on_error(error: web_sys::ErrorEvent) {
    log::warn!("Socket closed with an error\n{:?}", error);
}
