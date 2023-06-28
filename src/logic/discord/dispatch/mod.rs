pub mod guild_create;
pub mod ready;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum Event {
    #[serde(rename = "READY")]
    Ready(ready::ReadyData),
    #[serde(rename = "GUILD_CREATE")]
    GuildCreate(guild_create::GuildCreateData),
}
