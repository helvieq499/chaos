use std::rc::Rc;

use leptos::*;
use wasm_sockets::{EventClient, Message};

pub fn setup(cx: Scope) {
    let client = super::Client::get(cx);
    let gateway_url = super::gateway_url::resource(cx);

    let connect = client.connect;

    let socket = create_local_resource(
        cx,
        move || (connect.get(), gateway_url.read(cx).flatten()),
        move |params| {
            let client = client.clone();
            async move {
                match params {
                    (true, Some((from_local, url))) => {
                        log::debug!(
                            "Gateway URL: {url}\nFetched from {} source",
                            if from_local { "local" } else { "remote" }
                        );

                        EventClient::new(&url).map_or(None, |mut socket| {
                            socket.set_on_connection(Some(Box::new(on_connection)));
                            socket.set_on_message(Some(Box::new(move |socket, message| {
                                on_message(client.clone(), socket, message, cx);
                            })));
                            socket.set_on_close(Some(Box::new(on_close)));
                            socket.set_on_error(Some(Box::new(on_error)));

                            Some(Rc::new(socket))
                        })
                    }
                    _ => None,
                }
            }
        },
    );

    let (socket_signal, set_socket_signal) = create_signal(cx, None);
    create_effect(cx, move |_| set_socket_signal(socket.read(cx).flatten()));
    provide_context(cx, socket_signal);
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
