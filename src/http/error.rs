use reqwest::{Response, StatusCode, Url};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// An HTTP error.
#[derive(Debug, Error)]
pub enum Error {
    /// An invalid header value.
    #[error("invalid header value")]
    InvalidHeader,
    /// An error parsing a header value.
    #[error("failed to parse header value")]
    ParseHeaderValue,
    /// An unsuccessful request.
    #[error("request to \"{}\" failed: {}", .0.url, .0.status)]
    UnsuccessfulRequest(ErrorResponse),
    /// A request error.
    #[error(transparent)]
    Request(#[from] reqwest::Error),
}

#[derive(Clone, Debug)]
pub struct ErrorResponse {
    /// The HTTP status code of the response.
    pub status: StatusCode,
    /// The URL of the request.
    pub url: Url,
    /// A Discord JSON error object.
    pub error: Option<DiscordJsonError>,
}

impl ErrorResponse {
    pub(crate) async fn from_response(response: Response) -> ErrorResponse {
        ErrorResponse {
            status: response.status(),
            url: response.url().clone(),
            error: response.json().await.ok(),
        }
    }
}

/// A [Discord JSON error].
///
/// [Discord JSON error]: https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#json
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiscordJsonError {
    /// The error code.
    pub code: u32,
    /// A user friendly error message.
    pub message: String,
    #[serde(skip)]
    non_exhaustive: (),
}
