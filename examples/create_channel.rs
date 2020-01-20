use std::{env, fs};

use anyhow::Result;
use strife::builder::CreateChannel;
use strife::http::Http;
use strife::model::channel::guild::TextChannel;
use strife::model::id::GuildId;

#[tokio::main]
async fn main() -> Result<()> {
    // Get token from `DISCORD_TOKEN` environment variable or `.token` file.
    let token = env::var("DISCORD_TOKEN").or_else(|_| fs::read_to_string(".discord_token"))?;

    // Create an http client using our token.
    let http = Http::new(token);

    let guild_id = GuildId::from(668505301657583647);

    let channel: TextChannel = http
        .create_channel(
            guild_id,
            "example-channel",
            |channel: &mut CreateChannel<TextChannel>| {
                channel.topic("example topic");
            },
        )
        .await?;

    println!("channel: {:#?}", channel);

    Ok(())
}
