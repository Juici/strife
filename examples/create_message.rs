use std::{env, fs};

use anyhow::Result;
use strife::http::Http;
use strife::model::channel::Message;
use strife::model::color::Color;
use strife::model::id::ChannelId;

#[tokio::main]
async fn main() -> Result<()> {
    // Get token from `DISCORD_TOKEN` environment variable or `.token` file.
    let token = env::var("DISCORD_TOKEN").or_else(|_| fs::read_to_string(".discord_token"))?;

    // Create an http client using our token.
    let http = Http::new(token);

    let channel_id = ChannelId::from(668505302194192510);

    let msg: Message = http
        .create_message(channel_id, |m| {
            m.content("message content");
            m.tts(false);
            m.embed(|e| {
                e.title("embed title");
                e.description("embed description");
                e.timestamp_now();
                e.color(Color::new(0xFFA0FF));
                e.footer("embed footer", None);
                e.author("strife", None, None);
                e.fields(&[
                    ("field 1", "foo", true),
                    ("field 2", "bar", true),
                    ("field 3", "foobar", false),
                ]);
            });
        })
        .await?;

    println!("message: {:#?}", msg);

    Ok(())
}
