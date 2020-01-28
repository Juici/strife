use std::result::Result as StdResult;
use std::{fmt, io};

use serde_json::Error as JsonError;
use thiserror::Error;

use crate::http::HttpError;
use crate::model::image::ImageError;

/// The common result type returned by library functions.
pub type Result<T> = StdResult<T, Error>;

/// The common error enum returned by library functions within a [`Result`].
///
/// [`Result`]: type.Result.html
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    /// An error while formatting a message.
    #[error(transparent)]
    FormatError(#[from] fmt::Error),
    /// An error while performing an I/O operation.
    #[error(transparent)]
    IoError(#[from] io::Error),
    /// An HTTP error.
    #[error(transparent)]
    HttpError(#[from] HttpError),
    /// A JSON error.
    #[error(transparent)]
    JsonError(#[from] JsonError),
    /// An image error.
    #[error(transparent)]
    ImageError(#[from] ImageError),
}
