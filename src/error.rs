use std::{fmt, result};

use thiserror::Error;

/// The common result type returned by library functions.
pub type Result<T> = result::Result<T, Error>;

/// The common error enum returned by library functions within a [`Result`].
///
/// [`Result`]: type.Result.html
#[derive(Debug, Error)]
pub enum Error {
    /// An error while formatting a message.
    #[error("format error: {0}")]
    Format(#[from] fmt::Error),
}
