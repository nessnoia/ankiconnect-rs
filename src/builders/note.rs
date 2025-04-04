//! Builder for creating notes with a fluent interface

use crate::error::NoteError;
use crate::models::{FieldMedia, FieldRef, Media, MediaSource, Model, Note};
use std::collections::{HashMap, HashSet};

/// Builder for creating Anki notes
pub struct NoteBuilder {
    model: Model,
    field_values: HashMap<String, String>,
    tags: HashSet<String>,
    media: Vec<FieldMedia>,
}

impl NoteBuilder {
    /// Creates a new NoteBuilder for the given model
    pub fn new(model: Model) -> Self {
        Self {
            model,
            field_values: HashMap::new(),
            tags: HashSet::new(),
            media: Vec::new(),
        }
    }

    /// Add a field value with HTML escaping
    pub fn with_field(mut self, field_ref: FieldRef<'_>, content: &str) -> Self {
        let escaped_content = html_escape::encode_text(content).to_string();
        self.field_values
            .insert(field_ref.name().to_string(), escaped_content);
        self
    }

    /// Add a field value without HTML escaping
    pub fn with_field_raw(mut self, field_ref: FieldRef<'_>, content: &str) -> Self {
        self.field_values
            .insert(field_ref.name().to_string(), content.to_string());
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.insert(tag.to_string());
        self
    }

    /// Add audio to a specific field
    pub fn with_audio(self, field_ref: FieldRef<'_>, source: MediaSource, filename: &str) -> Self {
        self.with_media(field_ref, Media::audio(source, filename.to_string()))
    }

    /// Add image to a specific field
    pub fn with_image(self, field_ref: FieldRef<'_>, source: MediaSource, filename: &str) -> Self {
        self.with_media(field_ref, Media::image(source, filename.to_string()))
    }

    /// Add video to a specific field
    pub fn with_video(self, field_ref: FieldRef<'_>, source: MediaSource, filename: &str) -> Self {
        self.with_media(field_ref, Media::video(source, filename.to_string()))
    }

    /// Add generic media to a specific field
    pub fn with_media(mut self, field_ref: FieldRef<'_>, media: Media) -> Self {
        self.media.push(FieldMedia {
            media,
            field: field_ref.name().to_string(),
        });
        self
    }

    /// Build the note, validating all required fields are present
    pub fn build(self) -> Result<Note, NoteError> {
        Note::new(self.model, self.field_values, self.tags, self.media)
    }
}
