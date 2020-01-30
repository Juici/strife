use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{i64, str};

use bytes::Bytes;
use hyper::{HeaderMap, StatusCode};
use tokio::sync::{Mutex, RwLock};
use tokio::time::delay_for;

use crate::internal::prelude::*;

use super::prelude::*;

const RATELIMIT_GLOBAL: &str = "x-ratelimit-global";
const RATELIMIT_LIMIT: &str = "x-ratelimit-limit";
const RATELIMIT_REMAINING: &str = "x-ratelimit-remaining";

#[cfg(any(test, feature = "systime-ratelimits"))]
const RATELIMIT_RESET: &str = "x-ratelimit-reset";
#[cfg(any(test, not(feature = "systime-ratelimits")))]
const RATELIMIT_RESET_AFTER: &str = "x-ratelimit-reset-after";

const RETRY_AFTER: &str = "retry-after";

/// Ratelimiter for requests the the Discord REST API.
pub struct RateLimiter {
    token: Bytes,
    client: Arc<HyperClient>,
    global: Arc<Mutex<()>>,
    buckets: Arc<RwLock<HashMap<Bucket, Arc<Mutex<RateLimit>>>>>,
}

impl RateLimiter {
    /// Creates a new rate limit manager.
    pub fn new<T>(client: Arc<HyperClient>, token: T) -> RateLimiter
    where
        T: Into<Bytes>,
    {
        RateLimiter {
            token: token.into(),
            client,
            global: Default::default(),
            buckets: Default::default(),
        }
    }

    /// Performs a ratelimited request.
    pub async fn perform(&self, request: &Request<'_>) -> Result<HttpResponse> {
        log::trace!("performing request: {:#?}", request);

        let bucket = request.route.bucket();

        loop {
            // Build request and apply pre-request hooks before blocking on global
            // ratelimit, to reduce wasted time.

            // Build request with token.
            let req = request.build(self.token.clone())?;

            // Get the ratelimits for the bucket.
            let bucket_mtx = match bucket {
                // No ratelimits.
                Bucket::None => None,
                // Get the ratelimits for the bucket.
                bucket => {
                    // Since it's potentially costly to acquire a write lock when not needed, get a
                    // read lock and attempt to get the ratelimits mutex for the bucket.
                    let bucket_mtx = {
                        // Get a read lock on the bucket map.
                        let buckets = self.buckets.read().await;
                        // Get the ratelimits mutex for the bucket.
                        buckets.get(&bucket).cloned()
                    };

                    // If there was no ratelimits mutex for the bucket, we will have to create a new
                    // default one and insert it back into the map.
                    let bucket_mtx = match bucket_mtx {
                        Some(bucket_mtx) => bucket_mtx,
                        None => {
                            // Create a new default ratelimits mutex.
                            let bucket_mtx = Arc::new(Mutex::new(RateLimit::default()));
                            // Get a write lock on the bucket map.
                            let mut buckets = self.buckets.write().await;
                            // Insert the new ratelimits mutex for the bucket.
                            let _ = buckets.insert(bucket, bucket_mtx.clone());
                            bucket_mtx
                        }
                    };

                    // Apply pre-request hooks, awaiting ratelimits if required.
                    bucket_mtx.lock().await.pre_hook(&bucket).await;

                    Some(bucket_mtx)
                }
            };

            // Block if the global ratelimit has been reached by another thread.
            // Drop instantly to prevent blocking other threads.
            let _ = self.global.lock().await;

            // Send the request and await response.
            let response = self
                .client
                .request(req)
                .await
                .map_err(HttpError::HyperError)?;

            let bucket_mtx = match bucket_mtx {
                Some(bucket_mtx) => bucket_mtx,
                // No ratelimits apply to this request, return early.
                None => return Ok(response),
            };

            // Global ratelimit and bucket ratelimits are mutually exclusive, it is not
            // possible to hit both at the same time.

            // Check if global ratelimit was hit.
            if response.headers().contains_key(RATELIMIT_GLOBAL) {
                // Parse the retry-after header.
                match parse_header::<u64>(response.headers(), RETRY_AFTER)? {
                    Some(0) => {}
                    Some(retry_after) => {
                        // Instant before retrieving a lock on global ratelimit mutex.
                        let now = Instant::now();

                        // Lock on global ratelimit mutex to block other threads.
                        let _global = self.global.lock().await;

                        // The time elapsed since the instant before retrieving the lock global
                        // ratelimit mutex.
                        let elapsed = now.elapsed();
                        // The duration of the retry-after delay.
                        let delay = Duration::from_millis(retry_after);

                        /// A zero duration constant for comparison below.
                        const ZERO_DURATION: Duration = Duration::from_secs(0);

                        // Compare the ratelimit delay to the elapsed time.
                        match delay.checked_sub(elapsed) {
                            // The elapsed time is greater than or equal to the target delay.
                            None | Some(ZERO_DURATION) => {}
                            // Wait for the difference between the target delay and elapsed time.
                            Some(delay) => {
                                log::debug!("ratelimited globally for {}ms", retry_after);

                                // Wait for delay.
                                delay_for(delay).await;
                            }
                        }
                    }
                    // The global ratelimit was hit, but there is no retry-after header.
                    // This shouldn't happen and would be an error on the part of the Discord API.
                    // In this case, just return the response and log a warning.
                    #[cold]
                    None => {
                        log::warn!(
                            "global ratelimit hit, but no retry-after header found: {:#?}",
                            response
                        );

                        return Ok(response);
                    }
                }
            } else {
                // Apply post-request hooks and await bucket ratelimits.
                let resend = bucket_mtx
                    .lock()
                    .await
                    .post_hook(&response, &bucket)
                    .await?;

                // Request was sent, return the response.
                if !resend {
                    return Ok(response);
                }
            }

            // Request needs to be resent, go to start of next loop.
            continue;
        }
    }
}

struct RateLimit {
    limit: i64,
    remaining: i64,
    #[cfg(feature = "systime-ratelimits")]
    reset: i64,
    #[cfg(not(feature = "systime-ratelimits"))]
    reset_after: i64,
}

impl RateLimit {
    #[cfg(feature = "systime-ratelimits")]
    fn delay(&self) -> i64 {
        let now = chrono::Utc::now().timestamp_millis();
        self.reset - now
    }

    #[cfg(not(feature = "systime-ratelimits"))]
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
                "pre-emptive ratelimit on bucket {:?} for {}ms",
                bucket,
                delay
            );

            delay_for(Duration::from_millis(delay)).await;
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

        #[cfg(feature = "systime-ratelimits")]
        {
            if let Some(reset) = parse_header::<f64>(&response.headers(), RATELIMIT_RESET)? {
                self.reset = f64::ceil(reset * 1000f64) as i64;
            }
        }

        #[cfg(not(feature = "systime-ratelimits"))]
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
                log::debug!("ratelimited on route {:?} for {}ms", bucket, retry_after);
                delay_for(Duration::from_millis(retry_after)).await;

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
            #[cfg(feature = "systime-ratelimits")]
            reset: i64::MAX,
            #[cfg(not(feature = "systime-ratelimits"))]
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
    use std::iter::FromIterator;

    use hyper::header::{HeaderName, HeaderValue};

    use super::*;

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
