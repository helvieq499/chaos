use std::{rc::Rc, sync::RwLock};

use leptos::*;

use crate::logic::{client::ClientState, discord::RecvEvent, Client};

pub fn on_event(client: Rc<Client>, event: RecvEvent) {
    if let Ok(state) = serde_json::from_value::<ClientState>(event.data) {
        client.state.set(Some(RwLock::new(state)));
    }
}
