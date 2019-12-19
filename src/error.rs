use std::fmt::Error as FmtError;
use std::result::Result as StdResult;

use serde_json::Error as JsonError;
use thiserror::Error;

use crate::http::HttpError;

/// The common result type returned by library functions.
pub type Result<T> = StdResult<T, Error>;

/// The common error enum returned by library functions within a [`Result`].
///
/// [`Result`]: type.Result.html
#[derive(Debug, Error)]
pub enum Error {
    /// An error while formatting a message.
    #[error(transparent)]
    Format(#[from] FmtError),
    /// An HTTP error.
    #[error(transparent)]
    HttpError(#[from] HttpError),
    /// A JSON error.
    #[error(transparent)]
    JsonError(#[from] JsonError),
}
