use leptos::*;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Client {
    pub sequence: RefCell<Option<i32>>,
}

impl Client {
    fn new() -> Self {
        Self {
            sequence: RefCell::new(None),
        }
    }
}

pub fn setup(cx: Scope) {
    provide_context(cx, Client::new());
}
