use std::fmt::{self, Display};
use std::io::{self, BufRead, Cursor, Read, Seek, SeekFrom};

use crate::internal::prelude::Result;
use crate::model::image::ImageError;

/// An image format supported by the Discord API.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ImageFormat {
    /// An image in GIF format.
    Gif,
    /// An image in JPEG format.
    Jpeg,
    /// An image in PNG format.
    Png,
}

impl Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageFormat::Gif => f.write_str("gif"),
            ImageFormat::Jpeg => f.write_str("jpeg"),
            ImageFormat::Png => f.write_str("png"),
        }
    }
}

impl ImageFormat {
    /// Makes an educated guess about the format of the image, based on the
    /// magic bytes at the start of the buffer.
    pub(crate) fn from_slice(buffer: &[u8]) -> Result<ImageFormat> {
        guess_format(buffer)
    }

    pub(crate) fn from_reader<R>(reader: &mut R) -> Result<ImageFormat>
    where
        R: BufRead + Seek,
    {
        let mut start = [0; MAGIC_BYTES_BUF_LEN];

        // FIXME: Replace `seek(SeekFrom::Current(0))` with `stream_position()` when
        //        stabilized.

        // Get current position in reader.
        let cur = reader.seek(SeekFrom::Current(0))?;

        let len = io::copy(
            &mut reader.take(MAGIC_BYTES_BUF_LEN as u64),
            &mut Cursor::new(&mut start[..]),
        )? as usize;

        // Return to the start position in reader.
        reader.seek(SeekFrom::Start(cur))?;

        ImageFormat::from_slice(&start[..len])
    }
}

const MAGIC_BYTES_BUF_LEN: usize = 8 + 1;

#[rustversion::before(1.42)]
fn guess_format(buffer: &[u8]) -> Result<ImageFormat> {
    static MAGIC_BYTES: [(&[u8], ImageFormat); 4] = [
        (b"GIF87a", ImageFormat::Gif),
        (b"GIF89a", ImageFormat::Gif),
        (&[0xff, 0xd8, 0xff], ImageFormat::Jpeg),
        (b"\x89PNG\r\n\x1a\n", ImageFormat::Png),
    ];

    for &(bytes, format) in &MAGIC_BYTES {
        if buffer.starts_with(bytes) {
            return Ok(format);
        }
    }

    Err(ImageError::UnknownFormat.into())
}

#[rustversion::since(1.42)]
fn guess_format(buffer: &[u8]) -> Result<ImageFormat> {
    match buffer {
        [b'G', b'I', b'F', b'8', b'7', b'a', ..] | [b'G', b'I', b'F', b'8', b'9', b'a', ..] => {
            Ok(ImageFormat::Gif)
        }
        [0xff, 0xd8, 0xff, ..] => Ok(ImageFormat::Jpeg),
        [0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1a, b'\n', ..] => Ok(ImageFormat::Png),
        _ => Err(ImageError::UnknownFormat.into()),
    }
}
