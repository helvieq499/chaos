use leptos::*;
use std::{collections::HashMap, rc::Rc, sync::RwLock};

pub mod credentials;
pub use self::credentials::Credentials;
use super::types::Guild;

pub struct Client {
    pub connect: RwSignal<bool>,
    pub sequence: RwLock<Option<i32>>,
    pub credentials: RwSignal<Option<Credentials>>,

    pub guilds: RwSignal<RwLock<HashMap<u64, Rc<Guild>>>>,
}

impl Client {
    fn new(cx: Scope) -> Self {
        Self {
            connect: create_rw_signal(cx, false),
            sequence: RwLock::new(None),
            credentials: create_rw_signal(cx, Credentials::from_local_storage()),

            guilds: create_rw_signal(cx, RwLock::new(HashMap::new())),
        }
    }

    pub fn get(cx: Scope) -> Rc<Self> {
        use_context(cx).expect("to be provided")
    }
}

pub fn setup(cx: Scope) {
    provide_context(cx, Rc::new(Client::new(cx)));
}
