use thiserror::Error;

/// An image error.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ImageError {
    /// An error processing an image with unknown format.
    #[error("unknown image format")]
    UnknownFormat,
}
