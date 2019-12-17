//! Private re-exports of commonly used types in the http module.

pub use super::error::Error as HttpError;
pub use super::request::Request;
pub use super::routing::{Bucket, Method, Route};

pub type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;
pub type HyperClient = hyper::Client<HttpsConnector>;

pub type HttpRequest = hyper::Request<hyper::Body>;
pub type HttpResponse = hyper::Response<hyper::Body>;
