use crate::logic::{types::Guild, Client};
use leptos::*;
use std::rc::Rc;

#[derive(Debug, serde::Deserialize)]
pub struct GuildCreateData(pub Guild);

impl GuildCreateData {
    pub fn handle(self, client: Rc<Client>) {
        let mut extra_lock = self.0.extra.write().expect("unpoisoned");
        let mut channels_lock = self.0.channels.write().expect("unpoisoned");
        let old_channels = extra_lock.take().and_then(|extra| extra.channels);

        drop(extra_lock);

        if let Some(channels) = old_channels {
            for channel in channels {
                channels_lock.insert(channel.key(), Rc::new(channel));
            }
        }

        drop(channels_lock);

        client.guilds.update(|guilds| {
            guilds.set(self.0);
        });
    }
}
