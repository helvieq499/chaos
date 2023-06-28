use crate::logic::{client::Credentials, discord::RawPayload, socket::SocketType, Client};
use leptos::*;
use std::rc::Rc;
use wasm_sockets::ConnectionStatus;

#[derive(Debug, serde::Deserialize)]
pub struct HelloData {
    pub heartbeat_interval: u64,
}

impl HelloData {
    pub fn handle(self, cx: Scope, client: Rc<Client>) {
        log::debug!("Sending heartbeat every {} ms", self.heartbeat_interval);

        let socket = use_context::<SocketType>(cx).expect("to be provided");

        start_heartbeat_interval(client.clone(), socket, self.heartbeat_interval);

        client.credentials.with_untracked(move |creds| {
            if let Some(creds) = creds {
                socket.with_untracked(|socket| {
                    if let Some(socket) = socket {
                        let data = if creds.is_bot {
                            bot_identify(creds)
                        } else {
                            user_identify(creds)
                        };

                        serde_json::to_string(&RawPayload::new(2, data)).map_or_else(
                            |_| log::error!("Failed to serialize identify packet"),
                            |packet| {
                                if socket.send_string(&packet).is_err() {
                                    log::error!("Failed to send identify packet");
                                }
                            },
                        );
                    }
                });
            }
        });
    }
}

fn start_heartbeat_interval(client: Rc<Client>, socket: SocketType, interval: u64) {
    leptos::spawn_local(async move {
        let duration = std::time::Duration::from_millis(interval);

        loop {
            if let Ok(payload) = serde_json::to_string(&RawPayload::new(
                1,
                (*client.sequence.read().expect("not poisoned"))
                    .map_or(serde_json::Value::Null, |x| {
                        serde_json::Value::Number(x.into())
                    }),
            )) {
                if let Some(socket) = socket.get_untracked() {
                    if *socket.status.borrow() != ConnectionStatus::Connected {
                        return;
                    }

                    if socket.send_string(&payload).is_err() {
                        log::warn!("Failed to send heartbeat");
                    } else {
                        log::trace!("Sending heartbeat");
                    }
                } else {
                    return;
                }
            }

            futures_timer::Delay::new(duration).await;
        }
    });
}

fn bot_identify(creds: &Credentials) -> serde_json::Value {
    serde_json::json!({
        "token": creds.token,
        "properties": {
            "os": "Windows",
            "browser": "discord.py",
            "device": "discord.py",
        },
        "compress": false,
        "intents": 0b11_0001_0111_1110_1111_1101,
    })
}

// it would be better for everyone if these were documented
fn user_identify(creds: &Credentials) -> serde_json::Value {
    serde_json::json!({
        "capabilities": 8189,
        "client_state": {
            "api_code_version": 0,
            "guild_versions": {},
            "highest_last_message_id": "0",
            "private_channels_version": "0",
            "read_state_version": 0,
            "user_guild_settings_version": -1,
            "user_settings_version": -1,
        },
        "compress": false,
        "presence": {
            "activities": [],
            "afk": false,
            "since": 0,
            "status": "unknown",
        },
        "properties": {
            "browser": "Chrome",
            "browser_user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
            "browser_version": "114.0.0.0",
            "client_build_number": 208_319,
            "client_event_source": serde_json::Value::Null,
            "device": "",
            "os": "Windows",
            "os_version": "10",
            "referrer": "",
            "referrer_current": "",
            "referring_domain": "",
            "referring_domain_current": "",
            "release_channel": "stable",
            "system_locale": "en-US",
        },
        "token": creds.token
    })
}
