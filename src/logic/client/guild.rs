#[derive(Clone, Debug)]
pub enum Guild {
    Unavailable(UnavailableGuild),
    Available(AvailableGuild),
}

impl Guild {
    pub fn key(&self) -> u64 {
        match self {
            Self::Unavailable(x) => &x.id,
            Self::Available(x) => &x.id,
        }
        .parse()
        .unwrap_or(0)
    }
}

impl From<UnavailableGuild> for Guild {
    fn from(value: UnavailableGuild) -> Self {
        Self::Unavailable(value)
    }
}

impl From<AvailableGuild> for Guild {
    fn from(value: AvailableGuild) -> Self {
        Self::Available(value)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UnavailableGuild {
    pub id: String,
    pub unavailable: bool,
}

/// <https://discord.com/developers/docs/resources/guild#guild-object-guild-structure>
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AvailableGuild {
    pub id: String,
    pub name: String,
}
