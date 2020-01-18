use std::{env, fs};

use anyhow::Result;
use strife::http::unstable::{Request, Route};
use strife::http::Http;
use strife::model::user::ClientUser;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize pretty logging.
    pretty_env_logger::init();

    // Get token from `DISCORD_TOKEN` environment variable or `.token` file.
    let token = env::var("DISCORD_TOKEN").or_else(|_| fs::read_to_string(".discord_token"))?;

    // Create an http client using our token.
    let http = Http::new(token);

    // Request information on the current user.
    let user: ClientUser = http.request(Request::new(Route::GetCurrentUser)).await?;

    println!(
        "Client User: {name}#{discriminator} ({id})",
        name = user.name,
        discriminator = user.discriminator,
        id = user.id
    );

    Ok(())
}
