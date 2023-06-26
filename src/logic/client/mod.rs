use leptos::*;
use std::collections::HashMap;
use std::{rc::Rc, sync::RwLock};

pub mod credentials;
pub use self::credentials::Credentials;

pub mod state;
use self::guild::Guild;
pub use self::state::ClientState;

pub mod guild;

pub struct Client {
    pub connect: RwSignal<bool>,
    pub sequence: RwLock<Option<i32>>,
    pub credentials: RwSignal<Option<Credentials>>,

    pub initial_state: RwSignal<Option<RwLock<ClientState>>>,
    pub guilds: RwSignal<RwLock<HashMap<u64, Guild>>>,
}

impl Client {
    fn new(cx: Scope) -> Self {
        Self {
            connect: create_rw_signal(cx, false),
            sequence: RwLock::new(None),
            credentials: create_rw_signal(cx, Credentials::from_local_storage()),

            initial_state: create_rw_signal(cx, None),
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
