use bytes::Bytes;
use hyper::body::Body;
use hyper::header::{
    HeaderMap, HeaderValue, IntoHeaderName, AUTHORIZATION, CONTENT_TYPE, USER_AGENT,
};
use serde::Serialize;

use crate::constants;
use crate::internal::prelude::*;

use super::prelude::*;

const RATELIMIT_PRECISION: &str = "x-ratelimit-precision";

const APPLICATION_JSON: Bytes = Bytes::from_static(b"application/json");
const MILLISECOND: Bytes = Bytes::from_static(b"millisecond");

/// A request to be sent by the [`Http`] client.
///
/// # Stability
///
/// This is not part of the stable API and may change at any time. For a stable
/// API use the functions on the [`Http`] client.
///
/// [`Http`]: ../struct.Http.html
#[derive(Clone)]
pub struct Request<'a> {
    pub(crate) headers: Option<HeaderMap>,
    pub(crate) body: Option<Bytes>,
    pub(crate) route: Route<'a>,
}

impl<'a> Request<'a> {
    /// Creates an empty request for the given route.
    pub fn new(route: Route<'a>) -> Request {
        Request {
            headers: None,
            body: None,
            route,
        }
    }

    /// Sets the request headers.
    pub fn headers(&mut self, headers: HeaderMap) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    /// Adds a header to the request headers.
    ///
    /// Header names should be lowercase.
    pub fn header<K, V>(&mut self, key: K, value: V) -> Result<&mut Self>
    where
        K: IntoHeaderName,
        V: Into<Bytes>,
    {
        let value = value.into();
        let headers = self
            .headers
            .get_or_insert_with(|| HeaderMap::with_capacity(1));

        headers.insert(
            key,
            HeaderValue::from_maybe_shared(value.clone())
                .map_err(|_| HttpError::InvalidHeader(value))?,
        );
        Ok(self)
    }

    /// Adds a header to the request headers without validating the `value`
    /// contains no illegal bytes.
    ///
    /// See the safe function, [`header`], for more
    /// information.
    ///
    /// [`header`]: struct.Request.html#method.header
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that `value` contains
    /// no illegal bytes.
    pub unsafe fn header_unchecked<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: IntoHeaderName,
        V: Into<Bytes>,
    {
        let value = value.into();
        let headers = self
            .headers
            .get_or_insert_with(|| HeaderMap::with_capacity(1));

        headers.insert(key, HeaderValue::from_maybe_shared_unchecked(value));
        self
    }

    /// Sets the body of the request.
    pub fn body<B>(&mut self, body: B) -> &mut Self
    where
        B: Into<Bytes>,
    {
        self.body = Some(body.into());
        self
    }

    /// Serializes a value as JSON and sets it as the body of request.
    pub fn json<T>(&mut self, value: &T) -> Result<&mut Self>
    where
        T: ?Sized + Serialize,
    {
        let body = serde_json::to_vec(value)?;
        self.body = Some(Bytes::from(body));
        Ok(self)
    }

    pub(crate) fn build<T>(&self, token: T) -> Result<HttpRequest>
    where
        T: Into<Bytes>,
    {
        // Create header map.
        let headers = self.build_headers(token.into())?;

        // Build request.
        let route = &self.route;
        let body = self.body.clone().unwrap_or_default();

        let mut req = HttpRequest::new(Body::from(body));

        *req.method_mut() = route.method().into();
        *req.uri_mut() = route.url().parse().map_err(HttpError::InvalidUri)?;
        *req.headers_mut() = headers;

        Ok(req)
    }

    fn build_headers(&self, token: Bytes) -> Result<HeaderMap> {
        let mut headers = self.headers.clone().unwrap_or_default();

        // Reserve space for headers.
        headers.reserve(4);

        let auth = {
            let mut auth = HeaderValue::from_maybe_shared(token.clone())
                .map_err(|_| HttpError::InvalidHeader(token))?;
            auth.set_sensitive(true);
            auth
        };

        // Use unsafe `from_maybe_shared_unchecked` function to remove checks for
        // invalid bytes, since we can validate this before compile time.

        // SAFETY: "millisecond" contains no invalid bytes.
        let millisecond = unsafe { HeaderValue::from_maybe_shared_unchecked(MILLISECOND) };

        // Add base headers, cannot be overridden by custom headers.
        headers.insert(USER_AGENT, HeaderValue::from_static(constants::USER_AGENT));
        headers.insert(AUTHORIZATION, auth);
        headers.insert(RATELIMIT_PRECISION, millisecond);

        // Allow content-type to be overridden by custom headers.
        let _ = headers.entry(CONTENT_TYPE).or_insert_with(|| unsafe {
            // SAFETY: "application/json" contains no invalid bytes.
            HeaderValue::from_maybe_shared_unchecked(APPLICATION_JSON)
        });

        Ok(headers)
    }
}
