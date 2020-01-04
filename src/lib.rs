//! A lightweight library for the Discord API.

#![doc(html_root_url = "https://docs.rs/strife/*")]
#![deny(missing_docs)]

mod error;
#[macro_use]
mod internal;

pub mod client;
pub mod constants;
pub mod http;
pub mod model;

#[doc(inline)]
pub use crate::client::Client;
#[doc(inline)]
pub use crate::error::{Error, Result};
#[doc(inline)]
pub use crate::http::Http;
