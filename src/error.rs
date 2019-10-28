use std::{
    error::Error as StdError,
    fmt::{self, Display, Error as FormatError},
    result::Result as StdResult,
};

/// The common result type returned by library functions.
pub type Result<T> = StdResult<T, Error>;

/// The common error enum returned by library functions within a [`Result`].
///
/// [`Result`]: type.Result.html
#[derive(Debug)]
pub enum Error {
    /// An error while formatting a message.
    Format(FormatError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Format(ref inner) => inner.fmt(f),
        }
    }
}

impl StdError for Error {}
