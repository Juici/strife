//! The [`Client`] contains information about a single bots token and event
//! handlers. Dispatching of events to handlers and starting shard connections
//! are handled by the client.
//!
//! [`Client`]: struct.Client.html

mod event_handler;

use std::sync::{Arc, Mutex};

use crate::internal::prelude::*;

pub use self::event_handler::EventHandler;

/// A client that connects to Discord via the WebSocket and API.
pub struct Client {}

impl Client {
    /// Creates a Client for a bot user.
    ///
    /// Discord has a requirement of prefixing bot tokens with `"Bot "`, which
    /// this function will automatically add if not included.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// struct Handler;
    ///
    /// # use strife::client::EventHandler;
    /// impl EventHandler for Handler {}
    ///
    /// use std::env;
    /// use strife::client::Client;
    ///
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let token = env::var("DISCORD_TOKEN")?;
    /// let client = Client::new(&token, Handler)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<S, H>(token: S, _handler: H) -> Result<Client>
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


        // TODO: thread pool
        let _name = concat!(pkg_name!(), " client");
        // TODO: event handler

        // TODO: shard manager

        Ok(Client {})
    }
}
