use bytes::Bytes;
use hyper::http::uri::InvalidUri;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// An HTTP error.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    /// An Hyper error.
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    /// An invalid header value.
    #[error("invalid header value")]
    InvalidHeader(Bytes),
    /// Attempted to parse an invalid URI.
    #[error(transparent)]
    InvalidUri(#[from] InvalidUri),
    /// An error parsing a header.
    #[error("failed to parse header: {name}")]
    ParseHeaderError {
        /// The header name.
        name: String,
        /// The header value.
        value: Bytes,
    },
    /// An unsuccessful request.
    #[error("request to \"{}\" failed: {}", .0.url, .0.status)]
    UnsuccessfulRequest(ErrorResponse),
}

#[derive(Clone, Debug)]
pub struct ErrorResponse {
    /// The HTTP status code of the response.
    pub status: StatusCode,
    /// The URL of the request.
    pub url: String,
    /// A Discord JSON error object.
    pub error: Option<DiscordJsonError>,
}

/// A [Discord JSON error].
///
/// [Discord JSON error]: https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#json
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiscordJsonError {
    /// The error code.
    pub code: u32,
    /// A user friendly error message.
    pub message: String,
}
