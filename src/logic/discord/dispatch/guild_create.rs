use crate::logic::{types::Guild, Client};
use leptos::*;
use std::rc::Rc;

#[derive(Debug, serde::Deserialize)]
pub struct GuildCreateData(pub Guild);

impl GuildCreateData {
    pub fn handle(self, client: Rc<Client>) {
        client.guilds.update(|guilds| {
            let mut write_lock = guilds.write().expect("not poisoned");
            write_lock.insert(self.0.key(), Rc::new(self.0));
        });
    }
}
