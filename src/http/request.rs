use bytes::Bytes;
use hyper::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};

use crate::constants;
use crate::internal::prelude::*;

use super::prelude::*;

const RATELIMIT_PRECISION: &str = "x-ratelimit-precision";

const APPLICATION_JSON: Bytes = Bytes::from_static(b"application/json");
const MILLISECOND: Bytes = Bytes::from_static(b"millisecond");

#[derive(Clone)]
pub struct Request<'a> {
    pub headers: Option<HeaderMap>,
    pub body: Option<&'a [u8]>,
    pub route: Route<'a>,
}

impl<'a> Request<'a> {
    pub fn new(route: Route<'a>) -> Request {
        Request {
            headers: None,
            body: None,
            route,
        }
    }

    pub fn headers(&mut self, headers: HeaderMap) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    pub fn body(&mut self, body: &'a [u8]) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn build(&self, token: &str) -> Result<HttpRequest> {
        // Create header map.
        let mut headers = match &self.headers {
            Some(request_headers) => {
                let mut headers = HeaderMap::with_capacity(4 + request_headers.len());
                headers.extend(request_headers.clone());
                headers
            }
            None => HeaderMap::with_capacity(4),
        };

        use std::convert::TryInto;
        let mut auth: HeaderValue = token
            .try_into()
            .map_err(|_| HttpError::InvalidHeader(Bytes::copy_from_slice(token.as_bytes())))?;
        auth.set_sensitive(true);

        // Use unsafe `from_maybe_shared_unchecked` function to remove checks for
        // invalid bytes, since we can validate this before compile time.

        // SAFETY: "millisecond" contains no invalid bytes.
        let millisecond = unsafe { HeaderValue::from_maybe_shared_unchecked(MILLISECOND) };

        // Add base headers, cannot be overridden by custom headers.
        headers.insert(USER_AGENT, HeaderValue::from_static(constants::USER_AGENT));
        headers.insert(AUTHORIZATION, auth);
        headers.insert(RATELIMIT_PRECISION, millisecond);

        // Allow content type to be overridden by custom headers.
        if !headers.contains_key(CONTENT_TYPE) {
            // SAFETY: "application/json" contains no invalid bytes.
            let application_json =
                unsafe { HeaderValue::from_maybe_shared_unchecked(APPLICATION_JSON) };

            headers.insert(CONTENT_TYPE, application_json);
        }

        // Build request.
        let body = match self.body {
            Some(body) => Bytes::copy_from_slice(body),
            None => Bytes::new(),
        };
        let mut req = HttpRequest::new(body.into());

        let route = &self.route;

        *req.method_mut() = route.method().into();
        *req.uri_mut() = route.url().parse().map_err(HttpError::InvalidUri)?;
        *req.headers_mut() = headers;

        Ok(req)
    }
}
