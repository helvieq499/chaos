use crate::logic::{
    client::guild::{Guild, UnavailableGuild},
    Client,
};
use leptos::*;
use std::rc::Rc;

#[derive(Debug, serde::Deserialize)]
pub struct ReadyData {
    #[serde(rename = "v")]
    pub version: u8,
    pub user: serde_json::Value,
    #[serde(rename = "guilds")]
    pub partial_guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub application: serde_json::Value,
}

impl ReadyData {
    pub fn handle(&self, client: Rc<Client>) {
        if !self.partial_guilds.is_empty() {
            client.guilds.update(|guilds| {
                let mut write_lock = guilds.write().expect("not poisoned");

                for guild in &self.partial_guilds {
                    let guild: Guild = guild.clone().into();
                    write_lock.insert(guild.key(), Rc::new(guild));
                }
            });
        }
    }
}
