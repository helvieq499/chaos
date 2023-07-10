use leptos::*;
use std::{collections::HashMap, rc::Rc, sync::RwLock};

pub mod credentials;
pub use self::credentials::Credentials;
use super::types::Guild;

#[cfg(debug_assertions)]
pub const START_CONNECTED: bool = false;
#[cfg(not(debug_assertions))]
pub const START_CONNECTED: bool = true;

pub struct Client {
    pub connect: RwSignal<bool>,
    pub sequence: RwLock<Option<i32>>,
    pub credentials: RwSignal<Option<Credentials>>,

    pub guilds: RwSignal<RwLock<HashMap<u64, Rc<Guild>>>>,

    pub http: Rc<reqwest::Client>,
}

impl Client {
    fn new(cx: Scope) -> Self {
        Self {
            connect: create_rw_signal(cx, START_CONNECTED),
            sequence: RwLock::new(None),
            credentials: Credentials::new_signal_from_local_storage(cx),

            guilds: create_rw_signal(cx, RwLock::new(HashMap::new())),

            http: Rc::new(reqwest::Client::new()),
        }
    }

    pub fn get(cx: Scope) -> Rc<Self> {
        use_context(cx).expect("to be provided")
    }

    pub fn is_connected(&self) -> bool {
        // TODO: read the socket state
        self.connect.get()
    }
}

pub fn setup(cx: Scope) {
    provide_context(cx, Rc::new(Client::new(cx)));
}
