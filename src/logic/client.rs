use leptos::*;
use std::{rc::Rc, sync::RwLock};

pub struct Client {
    pub sequence: RwLock<Option<i32>>,
}

impl Client {
    fn new() -> Self {
        Self {
            sequence: RwLock::new(None),
        }
    }
}

pub fn setup(cx: Scope) {
    provide_context(cx, Rc::new(Client::new()));
}
