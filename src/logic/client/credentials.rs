#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Credentials {
    pub token: String,
    pub is_bot: bool,
    #[serde(default)]
    pub message_intent: bool,
}

impl Credentials {
    pub fn from_local_storage() -> Option<Self> {
        crate::utils::local_storage::get()
            .and_then(|local_storage| local_storage.get_item("credentials").ok().flatten())
            .and_then(|creds| serde_json::from_str(&creds).ok())
    }
}
