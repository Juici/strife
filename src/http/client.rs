use async_std::sync::Arc;

use bytes::Bytes;
use serde::de::DeserializeOwned;

use crate::internal::prelude::*;

use super::error::ErrorResponse;
use super::prelude::*;
use super::ratelimit::RateLimiter;

/// An HTTP client for performing requests to the REST API.
pub struct Http {
    /// Internal rate limit manager.
    ratelimiter: RateLimiter,
}

impl Http {
    /// Creates a new HTTP client with the given API token.
    pub fn new<S: AsRef<str>>(token: S) -> Http {
        // Trim whitespace from token.
        let token = token.as_ref().trim();
        // Add "Bot " prefix to token if necessary.
        let token = if token.starts_with("Bot ") {
            Bytes::copy_from_slice(token.as_bytes())
        } else {
            Bytes::from(format!("Bot {}", token))
        };

        let client = hyper::Client::builder().build(HttpsConnector::new());
        let client = Arc::new(client);

        Http {
            ratelimiter: RateLimiter::new(client, token),
        }
    }

    /// Performs a request with rate limiting if necessary.
    ///
    /// # Stability
    ///
    /// This is not part of the stable API and may change at any time.
    pub async fn request<T: DeserializeOwned>(&self, req: Request<'_>) -> Result<T> {
        json_body(&mut self.inner_request(req).await?).await
    }

    async fn inner_request(&self, request: Request<'_>) -> Result<HttpResponse> {
        let mut response = self.ratelimiter.perform(&request).await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(Error::HttpError(HttpError::UnsuccessfulRequest(
                ErrorResponse {
                    status: response.status(),
                    url: request.route.url().to_string(),
                    error: json_body(&mut response).await.ok(),
                },
            )))
        }
    }
}

async fn json_body<T: DeserializeOwned>(response: &mut HttpResponse) -> Result<T> {
    use bytes::buf::BufExt;

    let body = hyper::body::aggregate(response.body_mut())
        .await
        .map_err(HttpError::HyperError)?;
    let result: T = serde_json::from_reader(body.reader())?;
    Ok(result)
}
