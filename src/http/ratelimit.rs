use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use std::{i64, str};

use async_std::sync::{Arc, Mutex, RwLock};

use bytes::Bytes;
use futures_timer::Delay;
use hyper::{HeaderMap, StatusCode};

use crate::internal::prelude::*;

use super::prelude::*;
use super::request::Request;
use super::routing::Bucket;

const RATELIMIT_GLOBAL: &str = "x-ratelimit-global";
const RATELIMIT_LIMIT: &str = "x-ratelimit-limit";
const RATELIMIT_REMAINING: &str = "x-ratelimit-remaining";
#[cfg(any(test, feature = "systime_ratelimits"))]
const RATELIMIT_RESET: &str = "x-ratelimit-reset";
#[cfg(any(test, not(feature = "systime_ratelimits")))]
const RATELIMIT_RESET_AFTER: &str = "x-ratelimit-reset-after";

const RETRY_AFTER: &str = "Retry-After";

/// Ratelimiter for requests the the Discord REST API.
pub struct RateLimiter {
    token: String,
    client: Arc<HyperClient>,
    global: Arc<Mutex<()>>,
    routes: Arc<RwLock<HashMap<Bucket, Arc<Mutex<RateLimit>>>>>,
}

impl RateLimiter {
    /// Creates a new rate limit manager.
    pub fn new<S: AsRef<str>>(client: Arc<HyperClient>, token: S) -> RateLimiter {
        RateLimiter {
            token: token.as_ref().to_string(),
            client,
            global: Default::default(),
            routes: Default::default(),
        }
    }

    pub async fn perform(&self, request: &Request<'_>) -> Result<HttpResponse> {
        let bucket = request.route.bucket();

        loop {
            // Block if the global ratelimit has been reached by another thread.
            // Drop instantly to prevent blocking other threads.
            drop(self.global.lock().await);

            let req = request.build(&self.token)?;

            // No rate limits apply.
            if bucket == Bucket::None {}

            // Get the ratelimits for the bucket.
            let bucket_mtx = self.routes.write().await.entry(bucket).or_default().clone();

            // Apply pre-request hooks.
            bucket_mtx.lock().await.pre_hook(&bucket).await;

            let response = self
                .client
                .request(req)
                .await
                .map_err(HttpError::HyperError)?;

            // No ratelimits apply to this request.
            if bucket == Bucket::None {
                return Ok(response);
            }

            // Check if global ratelimit was hit.
            if response.headers().contains_key(RATELIMIT_GLOBAL) {
                // Lock on global ratelimit mutex to block other threads.
                let _global = self.global.lock().await;

                // Parse the retry-after header.
                match parse_header::<u64>(response.headers(), RETRY_AFTER)? {
                    Some(retry_after) => {
                        log::debug!("Ratelimited on bucket {:?} for {}ms", &bucket, retry_after);

                        // Wait for ratelimit delay.
                        Delay::new(Duration::from_millis(retry_after)).await;

                        // Request needs to be resent.
                        continue;
                    }
                    None => return Ok(response),
                }
            }

            // Check if bucket ratelimit was hit.
            let resend = {
                let mut lock = bucket_mtx.lock().await;
                lock.post_hook(&response, &bucket).await?
            };

            // Request was sent, return the response.
            if !resend {
                return Ok(response);
            }

            // Request needs to be resent.
        }
    }
}

pub struct RateLimit {
    limit: i64,
    remaining: i64,
    #[cfg(feature = "systime_ratelimits")]
    reset: i64,
    #[cfg(not(feature = "systime_ratelimits"))]
    reset_after: i64,
}

impl RateLimit {
    #[cfg(feature = "systime_ratelimits")]
    fn delay(&self) -> i64 {
        let now = chrono::Utc::now().timestamp_millis();
        self.reset - now
    }

    #[cfg(not(feature = "systime_ratelimits"))]
    fn delay(&self) -> i64 {
        self.reset_after
    }

    async fn pre_hook(&mut self, bucket: &Bucket) {
        if self.limit == 0 {
            return;
        }

        let delay = self.delay();
        if delay <= 0 {
            self.remaining = self.limit;
            return;
        }

        if self.remaining == 0 {
            // Delay is greater than zero.
            let delay = delay as u64;

            log::debug!(
                "Pre-emptive ratelimit on bucket {:?} for {}ms",
                bucket,
                delay
            );

            Delay::new(Duration::from_millis(delay)).await;
        }

        self.remaining -= 1;
    }

    async fn post_hook(&mut self, response: &HttpResponse, bucket: &Bucket) -> Result<bool> {
        if let Some(limit) = parse_header(&response.headers(), RATELIMIT_LIMIT)? {
            self.limit = limit;
        }

        if let Some(remaining) = parse_header(&response.headers(), RATELIMIT_REMAINING)? {
            self.remaining = remaining;
        }

        #[cfg(feature = "systime_ratelimits")]
        {
            if let Some(reset) = parse_header::<f64>(&response.headers(), RATELIMIT_RESET)? {
                self.reset = f64::ceil(reset * 1000f64) as i64;
            }
        }

        #[cfg(not(feature = "systime_ratelimits"))]
        {
            if let Some(reset_after) =
                parse_header::<f64>(&response.headers(), RATELIMIT_RESET_AFTER)?
            {
                self.reset_after = f64::ceil(reset_after * 1000f64) as i64;
            }
        }

        if response.status() != StatusCode::TOO_MANY_REQUESTS {
            return Ok(false);
        }

        match parse_header::<u64>(&response.headers(), RETRY_AFTER)? {
            Some(retry_after) => {
                log::debug!("Ratelimited on route {:?} for {}ms", bucket, retry_after);
                Delay::new(Duration::from_millis(retry_after)).await;

                Ok(true)
            }
            None => Ok(false),
        }
    }
}

impl Default for RateLimit {
    fn default() -> Self {
        RateLimit {
            limit: i64::MAX,
            remaining: i64::MAX,
            #[cfg(feature = "systime_ratelimits")]
            reset: i64::MAX,
            #[cfg(not(feature = "systime_ratelimits"))]
            reset_after: i64::MAX,
        }
    }
}

fn parse_header<T: FromStr>(headers: &HeaderMap, header: &str) -> Result<Option<T>> {
    let value = match headers.get(header) {
        Some(value) => value,
        None => return Ok(None),
    };

    let bytes = value.as_bytes();
    let s = str::from_utf8(bytes)
        .map_err(|_| HttpError::InvalidHeader(Bytes::copy_from_slice(bytes)))?;
    let value: T = s.parse().map_err(|_| HttpError::ParseHeaderError {
        name: header.to_string(),
        value: Bytes::copy_from_slice(bytes),
    })?;

    Ok(Some(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::header::{HeaderName, HeaderValue};
    use std::iter::FromIterator;

    #[test]
    fn test_parse_header() {
        let headers = HeaderMap::from_iter(vec![
            (
                HeaderName::from_static(RATELIMIT_LIMIT),
                HeaderValue::from_static("10"),
            ),
            (
                HeaderName::from_static(RATELIMIT_REMAINING),
                HeaderValue::from_static("3"),
            ),
            (
                HeaderName::from_static(RATELIMIT_RESET),
                HeaderValue::from_static("1470173022.420"),
            ),
            (
                HeaderName::from_static(RATELIMIT_RESET_AFTER),
                HeaderValue::from_static("6.457"),
            ),
        ]);

        assert_eq!(
            parse_header::<i64>(&headers, RATELIMIT_LIMIT)
                .unwrap()
                .unwrap(),
            10
        );
        assert_eq!(
            parse_header::<i64>(&headers, RATELIMIT_REMAINING)
                .unwrap()
                .unwrap(),
            3
        );
        assert_eq!(
            parse_header::<f64>(&headers, RATELIMIT_RESET)
                .unwrap()
                .unwrap(),
            1_470_173_022.420
        );
        assert_eq!(
            parse_header::<f64>(&headers, RATELIMIT_RESET_AFTER)
                .unwrap()
                .unwrap(),
            6.457
        );
    }
}
