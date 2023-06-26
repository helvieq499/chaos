use std::rc::Rc;

use crate::logic::{discord::RecvEvent, Client};

mod ready;

pub fn dispatch(client: Rc<Client>, event: RecvEvent) {
    if let Some(seq) = event.sequence {
        let mut lock = client.sequence.write().expect("not poisoned");
        *lock = Some(seq);
    }

    if let Some(typ) = &event.typ {
        match typ.as_str() {
            "READY" => ready::on_event(client, event),
            x => log::warn!("Unknown event dispatched {x}"),
        }
    }
}
