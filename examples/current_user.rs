use strife::http::unstable::{Request, Route};
use strife::model::user::ClientUser;
use strife::{Error, Http};

const DISCORD_TOKEN: &str = include_str!(".token");

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize pretty logging.
    pretty_env_logger::init();

    // Create an http client using our token.
    let http = Http::new(DISCORD_TOKEN);

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
