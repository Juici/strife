//! The HTTP module provides low-level functions for interacting with the
//! Discord API.
//!
//! Discord API endpoints are ratelimited to prevent spam. The library takes
//! preventative measures to ensure that requests are not ratelimited.

mod client;
mod error;
mod prelude;
mod ratelimit;
mod request;
mod routing;

pub use self::client::Http;
pub use self::error::Error as HttpError;
