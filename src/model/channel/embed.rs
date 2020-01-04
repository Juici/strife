use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::color::Color;

/// Embedded content in a [`Message`].
///
/// [`Message`]: ../struct.Message.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Embed {
    /// The title of the embed.
    pub title: Option<String>,
    /// The type of the embed.
    ///
    /// Always [`EmbedType::Rich`] for webhook embeds.
    ///
    /// [`EmbedType::Rich`]: enum.EmbedType.html#variant.Rich
    #[serde(rename = "type")]
    pub kind: EmbedType,
    /// The description of the embed.
    pub description: Option<String>,
    /// The URL of the embed.
    pub url: Option<String>,
    /// The timestamp of the embed.
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// The color code of the embed.
    #[serde(default, alias = "colour")]
    pub color: Color,
    /// The footer information of the embed.
    pub footer: Option<EmbedFooter>,
    /// The image information of the embed.
    pub image: Option<EmbedImage>,
    /// The thumbnail information of the embed.
    pub thumbnail: Option<EmbedThumbnail>,
    /// The video information of the embed.
    pub video: Option<EmbedVideo>,
    /// The provider information of the embed.
    pub provider: Option<EmbedProvider>,
    /// The author information of the embed.
    pub author: Option<EmbedAuthor>,
    /// The additional fields of the embed.
    #[serde(default)]
    pub fields: Vec<EmbedField>,
}

/// Type of an [`Embed`].
///
/// [`Embed`]: struct.Embed.html
#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum EmbedType {
    /// An article.
    #[serde(rename = "article")]
    Article,
    /// A `gifv` file.
    #[serde(rename = "gifv")]
    Gifv,
    #[serde(rename = "image")]
    /// An image.
    Image,
    /// A link.
    #[serde(rename = "link")]
    Link,
    /// A rich embed.
    #[serde(rename = "rich")]
    Rich,
    /// A video.
    #[serde(rename = "video")]
    Video,
}

impl Default for EmbedType {
    fn default() -> Self {
        EmbedType::Rich
    }
}

/// Footer information in an [`Embed`].
///
/// [`Embed`]: struct.Embed.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedFooter {
    /// The footer text.
    pub text: String,
    /// The URL of the footer icon.
    ///
    /// Only supports HTTP(S).
    pub icon_url: Option<String>,
    /// The proxied URL of the footer icon.
    pub proxy_icon_url: Option<String>,
}

/// Image information in an [`Embed`].
///
/// [`Embed`]: struct.Embed.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedImage {
    /// The URL of the image.
    ///
    /// Only supports HTTP(S).
    pub url: String,
    /// The proxied URL of the image.
    pub proxy_url: String,
    /// The height of the image.
    pub height: u64,
    /// The width of the image.
    pub width: u64,
}

/// Thumbnail information in an [`Embed`].
///
/// [`Embed`]: struct.Embed.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedThumbnail {
    /// The URL of the thumbnail.
    ///
    /// Only supports HTTP(S).
    pub url: String,
    /// The proxied URL of the thumbnail.
    pub proxy_url: String,
    /// The height of the thumbnail.
    pub height: u64,
    /// The width of the thumbnail.
    pub width: u64,
}

/// Video information in an [`Embed`].
///
/// [`Embed`]: struct.Embed.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedVideo {
    /// The URL of the video.
    pub url: String,
    /// The height of the video.
    pub height: u64,
    /// The width of the video.
    pub width: u64,
}

/// Provider information in an [`Embed`].
///
/// [`Embed`]: struct.Embed.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedProvider {
    /// The name of the provider.
    pub name: String,
    /// The URL of the provider.
    pub url: Option<String>,
}

/// Author information in an [`Embed`].
///
/// [`Embed`]: struct.Embed.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedAuthor {
    /// The author name.
    pub name: String,
    /// The URL of the author.
    pub url: Option<String>,
    /// The URL of the author icon.
    ///
    /// Only supports HTTP(S).
    pub icon_url: Option<String>,
    /// The proxied URL of the author icon.
    pub proxy_icon_url: Option<String>,
}

/// A field in an [`Embed`].
///
/// [`Embed`]: struct.Embed.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedField {
    /// The name of the field.
    pub name: String,
    /// The value of the embed.
    pub value: String,
    /// Whether the field should be displayed inline.
    #[serde(default)]
    pub inline: bool,
}
