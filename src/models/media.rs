//! Media model definitions

use std::path::PathBuf;

/// Types of media that can be attached to notes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    Audio,
    Video,
    Image,
}

/// Source of media content
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaSource {
    Path(PathBuf),
    Url(String),
    Base64(String),
}

impl MediaSource {
    /// Returns the path if this is a Path source
    pub fn path(&self) -> Option<&PathBuf> {
        match self {
            Self::Path(path) => Some(path),
            _ => None,
        }
    }

    /// Returns the URL if this is a URL source
    pub fn url(&self) -> Option<&String> {
        match self {
            Self::Url(url) => Some(url),
            _ => None,
        }
    }

    /// Returns the base64 data if this is a Base64 source
    pub fn data(&self) -> Option<&String> {
        match self {
            Self::Base64(data) => Some(data),
            _ => None,
        }
    }
}

/// Media attachment for a note
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Media {
    source: MediaSource,
    filename: String,
    media_type: MediaType,
}

impl Media {
    /// Creates a new media with the given source, filename, and type
    pub fn new(source: MediaSource, filename: String, media_type: MediaType) -> Self {
        Self {
            source,
            filename,
            media_type,
        }
    }

    /// Creates a new audio media
    pub fn audio(source: MediaSource, filename: String) -> Self {
        Self::new(source, filename, MediaType::Audio)
    }

    /// Creates a new image media
    pub fn image(source: MediaSource, filename: String) -> Self {
        Self::new(source, filename, MediaType::Image)
    }

    /// Creates a new video media
    pub fn video(source: MediaSource, filename: String) -> Self {
        Self::new(source, filename, MediaType::Video)
    }

    /// Gets the source of this media
    pub fn source(&self) -> &MediaSource {
        &self.source
    }

    /// Gets the filename of this media
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Gets the type of this media
    pub fn media_type(&self) -> MediaType {
        self.media_type
    }
}

/// Media attached to a specific field
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldMedia {
    pub(crate) media: Media,
    pub(crate) field: String,
}

impl FieldMedia {
    /// Gets the media
    pub fn media(&self) -> &Media {
        &self.media
    }

    /// Gets the field name
    pub fn field(&self) -> &str {
        &self.field
    }
}
