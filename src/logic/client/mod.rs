use leptos::*;
use std::{rc::Rc, sync::RwLock};

pub mod credentials;

pub struct Client {
    pub connect: RwSignal<bool>,
    pub sequence: RwLock<Option<i32>>,

    pub credentials: RwSignal<Option<credentials::Credentials>>,
}

impl Client {
    fn new(cx: Scope) -> Self {
        Self {
            connect: create_rw_signal(cx, false),
            sequence: RwLock::new(None),

            credentials: create_rw_signal(cx, None),
        }
    }

    pub fn get(cx: Scope) -> Rc<Self> {
        use_context(cx).expect("to be provided")
    }
}

pub fn setup(cx: Scope) {
    provide_context(cx, Rc::new(Client::new(cx)));
}
