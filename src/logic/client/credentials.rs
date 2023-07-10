use leptos::*;

use crate::utils::local_storage;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Credentials {
    pub token: String,
    pub is_bot: bool,
    #[serde(default)]
    pub message_intent: bool,
}

impl Credentials {
    pub fn new_signal_from_local_storage(cx: Scope) -> RwSignal<Option<Self>> {
        let signal = create_rw_signal(cx, Self::from_local_storage());

        create_effect(cx, move |prev| match prev {
            Some(_) => {
                if let Some(local_storage) = local_storage::get() {
                    local_storage
                        .set(
                            "credentials",
                            &serde_json::to_string(&signal.get()).expect("parsed successfully"),
                        )
                        .expect("persisted");
                }
            }
            None => signal.track(),
        });

        signal
    }

    pub fn from_local_storage() -> Option<Self> {
        crate::utils::local_storage::get()
            .and_then(|local_storage| local_storage.get_item("credentials").ok().flatten())
            .and_then(|creds| serde_json::from_str(&creds).ok())
    }
}
