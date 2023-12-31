use std::{collections::HashMap, rc::Rc, sync::RwLock};

mod member;
pub use member::Member;

#[derive(Debug, serde::Deserialize)]
pub struct Guild {
    pub id: String,
    pub unavailable: Option<bool>,

    #[serde(flatten)]
    pub info: Option<FullGuild>,
    properties: Option<FullGuild>,

    #[serde(flatten)]
    pub extra: RwLock<Option<GuildCreateExtra>>,

    #[serde(skip, default)]
    pub channels: RwLock<HashMap<u64, Rc<super::Channel>>>,
}

impl Guild {
    pub fn key(&self) -> u64 {
        self.id.parse::<u64>().expect("valid id")
    }

    pub fn normalize(mut self) -> Self {
        // migrate properties into info because the user api doesn't flatten it
        if let Some(props) = self.properties.take() {
            self.info = Some(props);
        }

        // migrate channels into hashmap
        let mut extra_lock = self.extra.write().expect("unpoisoned");
        let mut channels_lock = self.channels.write().expect("unpoisoned");
        let old_channels = extra_lock.take().and_then(|extra| extra.channels);

        drop(extra_lock);

        if let Some(channels) = old_channels {
            for channel in channels {
                channels_lock.insert(channel.key(), Rc::new(channel));
            }
        }

        drop(channels_lock);

        self
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct FullGuild {
    pub name: String,
    pub icon: Option<String>,
    // icon_hash: Option<String>,
    // splash: Option<String>,
    // discovery_splash: Option<String>,
    // owner: Option<bool>,
    // owner_id: String,
    // permissions: Option<String>,
    // region: Option<String>,
    // afk_channel_id: Option<String>,
    // afk_timeout: i32,
    // widget_enabled: Option<bool>,
    // widget_channel_id: Option<String>,
    // verification_level: u8,
    // default_message_notifications: u8,
    // explicit_content_filter: u8,
    // roles: Vec<serde_json::Value>,
    // emojis: Vec<serde_json::Value>,
    // features: Vec<serde_json::Value>,
    // mfa_level: u8,
    // application_id: Option<String>,
    // system_channel_id: Option<String>,
    // system_channel_flags: u8, // may need to be increased later
    // rules_channel_id: Option<String>,
    // max_presences: Option<u32>,
    // max_members: Option<u32>,
    // vanity_url_code: Option<String>,
    // description: Option<String>,
    // banner: Option<String>,
    // premium_tier: u8,
    // premium_subscription_count: Option<u32>,
    // preferred_locale: String,
    // public_updates_channel_id: Option<String>,
    // max_video_channel_users: Option<u32>,
    // max_stage_video_channel_users: Option<u32>,
    // approximate_member_count: Option<u32>,
    // approximate_presence_count: Option<u32>,
    // welcome_screen: Option<serde_json::Value>,
    // nsfw_level: u8,
    // stickers: Option<Vec<serde_json::Value>>,
    // premium_progress_bar_enabled: bool,
    // safety_alerts_channel_id: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct GuildCreateExtra {
    // pub joined_at: String,
    // pub large: bool,
    // pub member_count: u32,
    // pub voice_states: Vec<serde_json::Value>,
    // pub members: Vec<serde_json::Value>,
    pub channels: Option<Vec<super::Channel>>,
    // pub threads: Vec<serde_json::Value>,
    // pub presences: Vec<serde_json::Value>,
    // pub stage_instances: Vec<serde_json::Value>,
    // pub guild_scheduled_events: Vec<serde_json::Value>,
}
