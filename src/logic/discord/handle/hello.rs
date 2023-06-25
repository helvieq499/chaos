use leptos::*;
use std::rc::Rc;
use wasm_sockets::EventClient;

pub fn hello(
    client: Rc<crate::logic::Client>,
    event: &crate::logic::discord::RecvEvent,
    cx: Scope,
) {
    if let Some(heartbeat_interval) = event.data["heartbeat_interval"].as_u64() {
        log::debug!("Sending heartbeat every {heartbeat_interval} ms");

        let socket =
            use_context::<ReadSignal<Option<Rc<EventClient>>>>(cx).expect("to be provided");

        leptos::spawn_local(async move {
            let duration = std::time::Duration::from_millis(heartbeat_interval);

            loop {
                futures_timer::Delay::new(duration).await;

                let seq = *client.sequence.read().expect("not poisoned");
                let val = seq.map_or(serde_json::Value::Null, |x| {
                    serde_json::Value::Number(x.into())
                });

                if let Ok(payload) =
                    serde_json::to_string(&crate::logic::discord::RecvEvent::new(1, val))
                {
                    socket.with_untracked(|socket| {
                        if let Some(socket) = socket {
                            if socket.send_string(&payload).is_err() {
                                log::warn!("Failed to send heartbeat");
                            } else {
                                log::trace!("Sending heartbeat");
                            }
                        }
                    });
                }
            }
        });
    }
}
