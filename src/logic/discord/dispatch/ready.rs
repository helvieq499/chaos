use crate::logic::{types::Guild, Client};
use leptos::*;
use std::rc::Rc;

#[derive(Debug, serde::Deserialize)]
pub struct ReadyData {
    #[serde(rename = "v")]
    pub version: u8,
    pub user: serde_json::Value,
    pub guilds: Vec<Guild>,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub application: serde_json::Value,
}

impl ReadyData {
    pub fn handle(self, client: Rc<Client>) {
        if !self.guilds.is_empty() {
            client.guilds.update(|guilds| {
                let mut write_lock = guilds.write().expect("not poisoned");

                for guild in self.guilds {
                    write_lock.insert(guild.key(), Rc::new(guild));
                }
            });
        }
    }
}
