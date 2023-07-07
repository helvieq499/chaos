use crate::logic::client::Credentials;

pub trait RequestBuilderExt {
    fn auth(self, creds: &Credentials) -> Self;
}

impl RequestBuilderExt for reqwest::RequestBuilder {
    fn auth(self, creds: &Credentials) -> Self {
        if creds.is_bot {
            self.header("Authorization", format!("Bot {}", creds.token))
                .header("User-Agent", "DiscordBot (discord.py, 2.4.0a)")
        } else {
            self.header("Authorization", creds.token.clone())
        }
    }
}
