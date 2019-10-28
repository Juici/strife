//! A lightweight library for the Discord API.

#![doc(html_root_url = "https://docs.rs/strife/*")]
#![deny(missing_docs)]

mod error;
#[macro_use]
mod internal;

pub mod client;
pub mod constants;

pub use crate::client::Client;
pub use crate::error::{Error, Result};
