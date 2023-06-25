use std::rc::Rc;

pub fn dispatch(client: Rc<crate::logic::Client>, event: &crate::logic::discord::RecvEvent) {
    if let Some(seq) = event.sequence {
        let mut lock = client.sequence.write().unwrap();
        *lock = Some(seq);
    }
}
