use std::rc::Rc;

use leptos::*;
use wasm_sockets::{EventClient, Message};

pub fn setup(cx: Scope) {
    let client = use_context::<Rc<super::Client>>(cx).expect("to be provided");

    let socket_signal = create_rw_signal(cx, None);
    provide_context(cx, socket_signal);

    let gateway_url = super::gateway_url::resource(cx);
    create_effect(cx, move |_| {
        let client = client.clone();
        gateway_url.with(cx, move |res| {
            if let Some((from_local, url)) = res {
                log::debug!(
                    "Gateway URL: {url}\nFetched from {} source",
                    if *from_local { "local" } else { "remote" }
                );

                if let Ok(mut socket) = EventClient::new(url) {
                    socket.set_on_connection(Some(Box::new(on_connection)));
                    socket.set_on_message(Some(Box::new(move |socket, message| {
                        on_message(client.clone(), socket, message, cx);
                    })));
                    socket.set_on_close(Some(Box::new(on_close)));
                    socket.set_on_error(Some(Box::new(on_error)));

                    socket_signal.set(Some(Rc::new(socket)));
                }
            } else {
                log::error!("Failed to fetch gateway URL");
            }
        })
    });
}

fn on_connection(_socket: &EventClient) {
    log::info!("Connected to socket");
}

fn on_message(client: Rc<super::Client>, _socket: &EventClient, msg: Message, cx: Scope) {
    if let Message::Text(ref text) = msg {
        if let Ok(event) = serde_json::from_str::<super::discord::RecvEvent>(text) {
            log::trace!("{:?}", event);

            match event.opcode {
                0 => super::discord::handle::dispatch(client, &event),
                10 => super::discord::handle::hello(client, &event, cx),
                11 => (), // heartbeat acknowledge
                x => log::warn!("Unknown opcode {x}"),
            }
        } else {
            log::error!("Failed to parse message");
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
