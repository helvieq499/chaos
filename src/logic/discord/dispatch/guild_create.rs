use crate::logic::{
    client::guild::{AvailableGuild, Guild, UnavailableGuild},
    Client,
};
use leptos::*;
use std::rc::Rc;

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum GuildCreateData {
    Full(AvailableGuild),
    Partial(UnavailableGuild),
}

impl From<&GuildCreateData> for Guild {
    fn from(value: &GuildCreateData) -> Self {
        match value {
            GuildCreateData::Full(guild) => guild.clone().into(),
            GuildCreateData::Partial(guild) => guild.clone().into(),
        }
    }
}

impl GuildCreateData {
    pub fn handle(&self, client: Rc<Client>) {
        let guild: Guild = self.into();

        client.guilds.update(|guilds| {
            let mut write_lock = guilds.write().expect("not poisoned");
            write_lock.insert(guild.key(), Rc::new(guild));
        });
    }
}
