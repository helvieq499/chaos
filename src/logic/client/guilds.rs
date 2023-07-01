use crate::logic::types::Guild;
use std::{collections::HashMap, rc::Rc, sync::RwLock};

#[derive(Debug, Default)]
pub struct Guilds(pub RwLock<HashMap<u64, Rc<Guild>>>);

impl Guilds {
    pub fn get(&self, id: u64) -> Option<Rc<Guild>> {
        let lock = self.0.read().expect("unpoisoned");
        lock.get(&id).cloned()
    }

    pub fn set(&self, guild: Guild) {
        let mut lock = self.0.write().expect("unpoisoned");
        lock.insert(guild.key(), Rc::new(guild));
    }
}
