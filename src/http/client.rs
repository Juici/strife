use async_std::sync::Arc;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::internal::prelude::*;

use super::error::{Error as HttpError, ErrorResponse};
use super::ratelimit::RateLimiter;
use super::request::Request;

/// An HTTP client for performing requests to the REST API.
pub struct Http {
    /// Internal rate limit manager.
    ratelimiter: RateLimiter,
}

impl Http {
    /// Creates a new HTTP client with the given API token.
    pub fn new<S: AsRef<str>>(token: S) -> Http {
        let token = token.as_ref();

        // Panic if reqwest client could not be built.
        let client = Client::builder()
            .build()
            .expect("reqwest client could not be built");

        let client = Arc::new(client);

        Http {
            ratelimiter: RateLimiter::new(client, token.to_string()),
        }
    }

    /// Performs a request with rate limiting if necessary.
    pub async fn request<T: DeserializeOwned>(&self, req: Request<'_>) -> Result<T> {
        let response = self.inner_request(req).await?;
        let result = response.json().await;
        result.map_err(|err| Error::HttpError(HttpError::Request(err)))
    }

    async fn inner_request(&self, request: Request<'_>) -> Result<reqwest::Response> {
        let response = self.ratelimiter.perform(request).await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(Error::HttpError(HttpError::UnsuccessfulRequest(
                ErrorResponse::from_response(response).await,
            )))
        }
    }
}
