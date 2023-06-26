#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ClientState {
    #[serde(rename = "v")]
    pub version: u8,
    pub user: serde_json::Value,
    #[serde(rename = "guilds")]
    pub partial_guilds: Vec<super::guild::UnavailableGuild>,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub application: serde_json::Value,
}
