use std::rc::Rc;

use leptos::*;

use crate::logic::{
    client::guild::{AvailableGuild, Guild, UnavailableGuild},
    discord::RecvEvent,
    Client,
};

pub fn on_event(client: Rc<Client>, event: RecvEvent) {
    let guild = if event.data["unavailable"].as_bool().unwrap_or(false) {
        serde_json::from_value::<UnavailableGuild>(event.data).map(Guild::from)
    } else {
        serde_json::from_value::<AvailableGuild>(event.data).map(Guild::from)
    };

    guild.map_or_else(
        |err| log::warn!("Failed to parse guild\n{}", err),
        |guild| {
            client.guilds.update(|guilds| {
                let mut write_lock = guilds.write().expect("not poisoned");
                write_lock.insert(guild.key(), guild);
            });
        },
    );
}
