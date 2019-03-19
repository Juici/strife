//! The [`Client`] contains information about a single bots token and event
//! handlers. Dispatching of events to handlers and starting shard connections
//! are handled by the client.
//!
//! [`Client`]: struct.Client.html

mod event_handler;

use std::sync::{Arc, Mutex};

pub use self::event_handler::EventHandler;

/// A client that connects to Discord via the WebSocket and API.
pub struct Client {
    /// The token in use by the client.
    pub token: Arc<Mutex<String>>,
}

impl Client {
    /// Creates a Client for a bot user.
    ///
    /// Discord has a requirement of prefixing bot tokens with `"Bot "`, which
    /// this function will automatically add if not included.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::error::Error;
    /// use std::env;
    ///
    /// use strife::client::{Client, EventHandler};
    ///
    /// struct Handler;
    ///
    /// impl EventHandler for Handler {}
    ///
    /// fn main() -> Result<(), Box<Error>> {
    ///     let token = env::var("DISCORD_TOKEN")?;
    ///     let client = Client::new(&token, Handler);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new<S, H>(token: S, _handler: H) -> Client
    where
        S: AsRef<str>,
        H: EventHandler + Send + Sync + 'static,
    {
        let token = token.as_ref().trim();

        // Prepend "Bot " to token if required.
        let token = if token.starts_with("Bot ") {
            token.to_string()
        } else {
            format!("Bot {}", token)
        };

        let locked_token = Arc::new(Mutex::new(token));

        // TODO: thread pool
        // TODO: event handler

        // TODO: shard manager

        Client {
            token: locked_token,
        }
    }
}
