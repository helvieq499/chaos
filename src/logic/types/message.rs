#[derive(Clone, Debug, serde::Deserialize)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub author: serde_json::Value,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<serde_json::Value>,
    pub mention_roles: Vec<serde_json::Value>,
    pub mention_channels: Option<Vec<serde_json::Value>>,
    pub attachments: Option<Vec<serde_json::Value>>,
    pub embeds: Option<Vec<serde_json::Value>>,
    pub reactions: Option<Vec<serde_json::Value>>,
    pub nonce: Option<serde_json::Value>,
    pub pinned: bool,
    pub webhook_id: Option<String>,
    #[serde(rename = "type")]
    pub typ: u8,
    pub activity: Option<serde_json::Value>,
    pub application: Option<serde_json::Value>,
    pub application_id: Option<String>,
    pub message_reference: Option<serde_json::Value>,
    pub flags: Option<u32>,
    pub referenced_message: Option<serde_json::Value>,
    pub interaction: Option<serde_json::Value>,
    pub thread: Option<super::Channel>,
    pub components: Option<Vec<serde_json::Value>>,
    pub sticker_items: Option<Vec<serde_json::Value>>,
    pub stickers: Option<Vec<serde_json::Value>>,
    pub position: Option<i32>,
    pub role_subscription_data: Option<serde_json::Value>,

    #[serde(flatten)]
    pub extra: Option<MessageExtra>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct MessageExtra {
    pub guild_id: Option<String>,
    pub member: Option<serde_json::Value>,
    pub mentions: Vec<serde_json::Value>,
}
