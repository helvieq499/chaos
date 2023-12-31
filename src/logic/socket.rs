use super::discord::{Event, Payload};
use crate::components::infobar::missing_intents::Enable as MissingIntentsWarning;
use crate::logic::discord::{dispatch, gateway};
use leptos::*;
use std::rc::Rc;
use wasm_sockets::{EventClient, Message};

pub type SocketType = ReadSignal<Option<Rc<EventClient>>>;

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
                        let url = format!("{}/?encoding=json&v=10", url);

                        log::debug!(
                            "Gateway URL: {url}\nFetched from {} source",
                            if from_local { "local" } else { "remote" }
                        );

                        EventClient::new(&url).map_or(None, |mut socket| {
                            socket.set_on_connection(Some(Box::new(on_connection)));
                            socket.set_on_message(Some(Box::new(move |socket, message| {
                                on_message(client.clone(), socket, message, cx);
                            })));
                            socket.set_on_close(Some(Box::new(move |event| on_close(event, cx))));
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
    provide_context::<SocketType>(cx, socket_signal);
}

fn on_connection(_socket: &EventClient) {
    log::info!("Connected to socket");
}

fn on_message(client: Rc<super::Client>, _socket: &EventClient, msg: Message, cx: Scope) {
    match msg {
        Message::Text(text) => {
            log::trace!("{}", text);

            if let Ok(payload) = serde_json::from_str::<Payload>(&text) {
                match payload.event {
                    Event::DispatchEvent(event) => match event {
                        dispatch::Event::GuildCreate(data) => data.handle(client),
                        dispatch::Event::Ready(data) => data.handle(client),
                        dispatch::Event::MessageCreate(data) => data.handle(client),
                    },
                    Event::GatewayEvent(event) => match event {
                        gateway::Event::Hello { data, .. } => data.handle(cx, client),
                        gateway::Event::HeartbeatAck { .. } => (), // expected behavior: disconnect if this is not received
                        ev => log::warn!("Unknown gateway event {:?}", ev),
                    },
                    Event::Raw(event) => log::debug!(
                        "Unknown event op[{}] t[{}]",
                        event.opcode,
                        event.typ.unwrap_or_default()
                    ),
                }
            }
        }
        Message::Binary(_) => log::warn!("Gateway sent an unknown binary message"),
    }
}

fn on_close(close: web_sys::CloseEvent, cx: Scope) {
    const CLOSED: &str = "Socket closed:";

    let code = close.code();

    match code {
        4008 => log::warn!("{CLOSED} rate limited"),
        4009 => log::warn!("{CLOSED} timed out"),
        4011 => log::warn!("{CLOSED} bot too big"),
        // TODO: show a notification that the intent is not enabled
        4014 => {
            log::warn!("{CLOSED} disallowed intent");
            use_context::<RwSignal<MissingIntentsWarning>>(cx)
                .expect("provided")
                .set(MissingIntentsWarning);
        }
        code => log::error!("{CLOSED} {} ({code})", close.reason()),
    }
}

fn on_error(error: web_sys::ErrorEvent) {
    log::warn!("Socket closed with an error\n{:?}", error);
}
