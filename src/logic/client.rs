use leptos::*;
use std::{rc::Rc, sync::RwLock};

pub struct Client {
    pub connect: RwSignal<bool>,
    pub sequence: RwLock<Option<i32>>,
}

impl Client {
    fn new(cx: Scope) -> Self {
        Self {
            connect: create_rw_signal(cx, false),
            sequence: RwLock::new(None),
        }
    }
}

pub fn setup(cx: Scope) {
    provide_context(cx, Rc::new(Client::new(cx)));
}
