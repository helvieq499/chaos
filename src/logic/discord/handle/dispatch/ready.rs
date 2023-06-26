use std::{rc::Rc, sync::RwLock};

use leptos::*;

use crate::logic::{
    client::{guild::Guild, ClientState},
    discord::RecvEvent,
    Client,
};

pub fn on_event(client: Rc<Client>, event: RecvEvent) {
    if let Ok(state) = serde_json::from_value::<ClientState>(event.data) {
        if !state.partial_guilds.is_empty() {
            client.guilds.update(|guilds| {
                let mut write_lock = guilds.write().expect("not poisoned");

                for guild in &state.partial_guilds {
                    let guild: Guild = guild.clone().into();
                    write_lock.insert(guild.key(), guild);
                }
            });
        }

        client.initial_state.set(Some(RwLock::new(state)));
    }
}
