use crate::logic::{types::Message, Client};
use leptos::*;
use std::rc::Rc;

#[derive(Debug, serde::Deserialize)]
pub struct MessageCreateData(pub Message);

impl MessageCreateData {
    pub fn handle(self, client: Rc<Client>) {
        let channel = self.0.extra.as_ref().and_then(|extra| {
            extra
                .guild_id
                .as_ref()
                .and_then(|guild_id| guild_id.parse::<u64>().ok())
                .and_then(|guild_id| client.guilds.with_untracked(|guilds| guilds.get(guild_id)))
                .and_then(|guild| {
                    let channels_lock = guild.channels.read().expect("unpoisoned");
                    channels_lock
                        .get(&self.0.channel_id.parse::<u64>().expect("valid id"))
                        .cloned()
                })
        });

        if let Some(channel) = channel {
            let mut messages_lock = channel.messages.write().expect("unpoisoned");
            messages_lock.push(Rc::new(self.0));
        }
    }
}
