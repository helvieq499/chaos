pub mod guild_create;
pub mod message_create;
pub mod ready;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum Event {
    #[serde(rename = "READY")]
    Ready(ready::ReadyData),
    #[serde(rename = "GUILD_CREATE")]
    GuildCreate(guild_create::GuildCreateData),
    #[serde(rename = "MESSAGE_CREATE")]
    MessageCreate(message_create::MessageCreateData),
}
