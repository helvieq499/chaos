#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Credentials {
    pub token: String,
    pub is_bot: bool,
}
