use std::path::{Path, PathBuf};

use strife::model::image::{ImageData, ImageDataRef, ImageFormat};

macro_rules! data_uri {
    ($ext:literal) => {
        concat!(
            "data:image/",
            $ext,
            ";base64,",
            include_str!(concat!("images/avatar.", $ext, ".base64"))
        )
    };
}

fn test_path(format: ImageFormat) -> PathBuf {
    const TEST_FILE: &str = file!();

    let cur_dir = Path::new(TEST_FILE)
        .parent()
        .expect("failed to get test directory");

    let mut path = cur_dir.join("images/avatar");
    path.set_extension(format.to_string());

    if path.is_file() {
        path
    } else {
        panic!("missing avatar test file: {}", path.display());
    }
}

#[test]
fn test_gif() {
    const DATA: &str = data_uri!("gif");

    let raw = unsafe { ImageData::from_data_uri_unchecked(DATA) };

    let path = test_path(ImageFormat::Gif);
    let image = ImageData::open(path).unwrap();

    assert_eq!(raw, image);
}

#[test]
fn test_jpeg() {
    const DATA: &str = data_uri!("jpeg");

    let raw = unsafe { ImageData::from_data_uri_unchecked(DATA) };

    let path = test_path(ImageFormat::Jpeg);
    let image = ImageData::open(path).unwrap();

    assert_eq!(raw, image);
}

#[test]
fn test_png() {
    const DATA: &str = data_uri!("png");

    let raw = unsafe { ImageData::from_data_uri_unchecked(DATA) };

    let path = test_path(ImageFormat::Png);
    let image = ImageData::open(path).unwrap();

    assert_eq!(raw, image);
}

#[test]
fn test_deref() {
    const DATA: &str = data_uri!("png");

    let raw1 = unsafe { ImageData::from_data_uri_unchecked(DATA) };
    let raw2 = unsafe { ImageDataRef::from_data_uri_unchecked(DATA) };

    assert_eq!(&*raw1, raw2);

    let implicit: &ImageDataRef = &raw1;
    assert_eq!(implicit, raw2);
}

#[test]
fn test_eq() {
    const DATA: &str = data_uri!("png");

    let raw1 = unsafe { ImageData::from_data_uri_unchecked(DATA) };
    let raw2 = unsafe { ImageDataRef::from_data_uri_unchecked(DATA) };

    assert_eq!(raw1, raw2);
    assert_eq!(raw2, raw1);

    assert_eq!(raw2, DATA);
    assert_eq!(DATA, raw2);

    assert_eq!(raw1, DATA);
    assert_eq!(DATA, raw1);
}
