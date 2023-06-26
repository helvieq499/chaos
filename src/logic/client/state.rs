#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ClientState {
    #[serde(rename = "v")]
    version: u8,
    user: serde_json::Value,
    guilds: Vec<serde_json::Value>,
    session_id: String,
    resume_gateway_url: String,
    application: serde_json::Value,
}
