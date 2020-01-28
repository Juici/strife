//! Models [image data] matching the [scheme] supported by the Discord API.
//!
//! [image data]: https://discordapp.com/developers/docs/reference#image-data
//! [scheme]: https://en.wikipedia.org/wiki/Data_URI_scheme

mod error;
mod format;

use std::borrow::Cow;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};
use std::path::Path;

use base64::display::Base64Display;
use serde::{Deserialize, Serialize};

use crate::internal::prelude::*;

pub use self::error::ImageError;
pub use self::format::ImageFormat;

// TODO: Add async support?

/// Image data supported by the Discord API.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ImageData(Cow<'static, str>);

impl ImageData {
    /// Creates image data from a byte slice.
    pub fn from_slice(buffer: &[u8], format: ImageFormat) -> Result<ImageData> {
        let encoded = Base64Display::with_config(buffer, base64::STANDARD);

        let s = format!("data:image/{};base64,{}", format, encoded);
        Ok(ImageData(Cow::Owned(s)))
    }

    /// Creates image data from a byte slice.
    ///
    /// Makes an educated guess about the image format.
    pub fn from_slice_guess_format(buffer: &[u8]) -> Result<ImageData> {
        let format = ImageFormat::from_slice(buffer)?;
        Self::from_slice(buffer, format)
    }

    /// Creates image data from a reader.
    pub fn from_reader<R>(mut reader: R, format: ImageFormat) -> Result<ImageData>
    where
        R: BufRead + Seek,
    {
        // FIXME: Use a stream based approach reduce allocation.

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        Self::from_slice(&buf, format)
    }

    /// Creates image data from a reader.
    ///
    /// Makes an educated guess about the image format.
    pub fn from_reader_guess_format<R>(mut reader: R) -> Result<ImageData>
    where
        R: BufRead + Seek,
    {
        let format = ImageFormat::from_reader(&mut reader)?;
        Self::from_reader(reader, format)
    }

    /// Creates image data from the image located at the specified path.
    ///
    /// Makes an educated guess about the image format based on file contents
    /// and path extension.
    pub fn open<P>(path: P) -> Result<ImageData>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;

        // Attempt to get initial capacity to allocate to buffer for reading file.
        let initial_buf_len = file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0);

        let mut reader = BufReader::new(file);
        let format = ImageFormat::from_reader(&mut reader)?;

        // FIXME: Use a stream based approach reduce allocation.

        let mut buf = Vec::with_capacity(initial_buf_len);
        reader.read_to_end(&mut buf)?;

        Self::from_slice(&buf, format)
    }

    /// Creates image data from a [Data URI scheme].
    ///
    /// Supports GIF, JPEG and PNG formats.
    ///
    /// An example Data URI format:
    /// ```text
    /// data:image/jpeg;base64,BASE64_ENCODED_JPEG_IMAGE_DATA
    /// ```
    ///
    /// # Safety
    ///
    /// No checks are made that Data URI scheme is valid and supported, as a
    /// result it is possible for the Discord API to to return errors due to
    /// invalid an image format.
    pub const unsafe fn from_data_uri_unchecked(data: &'static str) -> ImageData {
        ImageData(Cow::Borrowed(data))
    }
}

impl AsRef<str> for ImageData {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<ImageData> for String {
    fn from(image: ImageData) -> Self {
        image.0.into_owned()
    }
}

impl From<ImageData> for Cow<'static, str> {
    fn from(image: ImageData) -> Self {
        image.0
    }
}

impl PartialEq<str> for ImageData {
    fn eq(&self, other: &str) -> bool {
        self.0[..] == other[..]
    }
}
