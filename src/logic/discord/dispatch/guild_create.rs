use crate::{
    logic::{types::Guild, Client},
    utils::collection_ext::CollectionExt,
};
use leptos::*;
use std::rc::Rc;

#[derive(Debug, serde::Deserialize)]
pub struct GuildCreateData(pub Guild);

impl GuildCreateData {
    pub fn handle(self, client: Rc<Client>) {
        client.guilds.update(|guilds| {
            guilds.set(self.0.key(), Rc::new(self.0.normalize()));
        });
    }
}
