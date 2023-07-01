#[derive(Clone, Debug, serde::Deserialize)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub typ: u8,
    pub guild_id: Option<String>,
    pub position: Option<u8>,
    pub permission_overwrites: Option<Vec<serde_json::Value>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u32>,
    pub rate_limit_per_user: Option<u16>,
    pub recipients: Option<Vec<serde_json::Value>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub managed: Option<bool>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<String>,
    pub message_count: Option<u64>,
    pub member_count: Option<u8>,
    pub thread_metadata: Option<serde_json::Value>,
    pub member: Option<serde_json::Value>,
    pub default_auto_archive_duration: Option<u16>,
    pub permissions: Option<String>,
    pub flags: Option<u8>,
    pub total_message_sent: Option<u64>,
    pub available_tags: Option<Vec<serde_json::Value>>,
    pub applied_tags: Option<Vec<String>>,
    pub default_reaction_emoji: Option<serde_json::Value>,
    pub default_thread_rate_limit_per_user: Option<u16>,
    pub default_sort_order: Option<u8>,
    pub default_forum_layout: Option<u8>,
}

impl Channel {
    pub fn key(&self) -> u64 {
        self.id.parse::<u64>().expect("valid id")
    }
}
