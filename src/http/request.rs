use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::{Client, RequestBuilder};

use crate::constants;
use crate::internal::prelude::*;

use super::routing::Route;
use super::HttpError;

const RATELIMIT_PRECISION: &str = "x-ratelimit-precision";

const APPLICATION_JSON: &str = "application/json";
const MILLISECOND: &str = "millisecond";

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

    pub fn build(&self, client: &Client, token: &str) -> Result<RequestBuilder> {
        let route = &self.route;

        let method = route.method();
        let url = route.url();

        let mut builder = client.request(method.into(), &*url);

        if let Some(body) = self.body {
            builder = builder.body(body.to_vec());
        }

        let mut headers = match &self.headers {
            Some(request_headers) => {
                let mut headers = HeaderMap::with_capacity(4 + request_headers.len());
                headers.extend(request_headers.clone());
                headers
            }
            None => HeaderMap::with_capacity(4),
        };

        let mut auth = HeaderValue::from_str(token).map_err(|_| HttpError::InvalidHeader)?;
        auth.set_sensitive(true);

        headers.insert(USER_AGENT, HeaderValue::from_static(constants::USER_AGENT));
        headers.insert(AUTHORIZATION, auth);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static(APPLICATION_JSON));
        headers.insert(RATELIMIT_PRECISION, HeaderValue::from_static(MILLISECOND));

        Ok(builder.headers(headers))
    }
}
