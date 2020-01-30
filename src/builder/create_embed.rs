use chrono::{DateTime, TimeZone, Utc};
use serde::Serialize;

use crate::model::color::Color;

/// A builder for creating a new message.
#[derive(Debug, Serialize)]
pub struct CreateEmbed<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<CreateEmbedFooter<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<CreateEmbedImage<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<CreateEmbedThumbnail<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<CreateEmbedAuthor<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fields: Vec<CreateEmbedField<'a>>,
}

// TODO: Add support for attachments `attachment://filename.png`.

impl<'a> CreateEmbed<'a> {
    pub(crate) fn new() -> Self {
        CreateEmbed {
            title: None,
            description: None,
            url: None,
            timestamp: None,
            color: None,
            footer: None,
            image: None,
            thumbnail: None,
            author: None,
            fields: Vec::new(),
        }
    }

    /// Sets the title of the embed.
    pub fn title(&mut self, title: &'a str) {
        self.title = Some(title);
    }

    /// Sets the description of the embed.
    pub fn description(&mut self, description: &'a str) {
        self.description = Some(description);
    }

    /// Sets the URL of the embed.
    pub fn url(&mut self, url: &'a str) {
        self.url = Some(url);
    }

    /// Sets the timestamp of the embed.
    pub fn timestamp<Tz: TimeZone>(&mut self, timestamp: DateTime<Tz>) {
        self.timestamp = Some(timestamp.with_timezone(&Utc));
    }

    /// Sets the timestamp of the embed to the current time.
    pub fn timestamp_now(&mut self) {
        self.timestamp = Some(Utc::now());
    }

    /// Sets the color of the embed.
    pub fn color(&mut self, color: Color) {
        self.color = Some(color);
    }

    /// Sets the embed footer information.
    pub fn footer(&mut self, text: &'a str, icon_url: Option<&'a str>) {
        self.footer = Some(CreateEmbedFooter { text, icon_url });
    }

    /// Sets the embed image.
    pub fn image(&mut self, image_url: &'a str) {
        self.image = Some(CreateEmbedImage { url: image_url });
    }

    /// Sets the embed thumbnail image.
    pub fn thumbnail(&mut self, thumbnail_url: &'a str) {
        self.thumbnail = Some(CreateEmbedThumbnail { url: thumbnail_url });
    }

    /// Sets the embed author information.
    pub fn author(&mut self, name: &'a str, url: Option<&'a str>, icon_url: Option<&'a str>) {
        self.author = Some(CreateEmbedAuthor {
            name,
            url,
            icon_url,
        });
    }

    /// Adds a field to the embed.
    pub fn field(&mut self, name: &'a str, value: &'a str) {
        self.fields.push(CreateEmbedField {
            name,
            value,
            inline: false,
        });
    }

    /// Adds an inline field to the embed.
    pub fn field_inline(&mut self, name: &'a str, value: &'a str) {
        self.fields.push(CreateEmbedField {
            name,
            value,
            inline: true,
        });
    }

    /// Adds fields to the embed.
    ///
    /// # Notes
    ///
    /// Prefer this function over [`field`] or [`field_inline`] when adding
    /// multiple fields, due to the reduced number of reallocations as
    /// more fields are added.
    ///
    /// [`field`]: #method.field
    /// [`field_inline`]: #method.field_inline
    pub fn fields<I>(&mut self, fields: I)
    where
        I: IntoIterator<Item = &'a (&'a str, &'a str, bool)>,
    {
        self.fields.extend(
            fields
                .into_iter()
                .map(|&(name, value, inline)| CreateEmbedField {
                    name,
                    value,
                    inline,
                }),
        );
    }
}

#[derive(Debug, Serialize)]
pub struct CreateEmbedFooter<'a> {
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct CreateEmbedImage<'a> {
    url: &'a str,
}

#[derive(Debug, Serialize)]
pub struct CreateEmbedThumbnail<'a> {
    url: &'a str,
}

#[derive(Debug, Serialize)]
pub struct CreateEmbedAuthor<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct CreateEmbedField<'a> {
    name: &'a str,
    value: &'a str,
    inline: bool,
}
