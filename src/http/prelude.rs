//! Private re-exports of commonly used types in the http module.

pub use super::error::Error as HttpError;
pub use super::request::Request;
pub use super::routing::{Bucket, Method, Route};

// Require one of the `native-tls` and `rustls-tls` features to be enabled.
cfg_if::cfg_if! {
    if #[cfg(feature = "rustls-tls")] {
        pub type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
    } else if #[cfg(feature = "native-tls")] {
        pub type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;
    } else {
        compile_error!("one of the `rustls-tls` and `native-tls` features must be enabled");

        // Hacky approach to only show the relevant compile error.
        pub type HttpsConnector = hyper::client::HttpConnector;
    }
}

pub type HyperClient = hyper::Client<HttpsConnector>;

pub type HttpRequest = hyper::Request<hyper::Body>;
pub type HttpResponse = hyper::Response<hyper::Body>;
