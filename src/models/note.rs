//! Note model definitions

use crate::error::NoteError;
use crate::models::{FieldMedia, Model};
use crate::Media;
use std::collections::{HashMap, HashSet};

/// Unique identifier for an Anki note
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoteId(pub u64);

impl NoteId {
    /// Gets the raw ID value
    pub fn value(&self) -> u64 {
        self.0
    }
}

/// Represents a note in Anki
#[derive(Debug, Clone)]
pub struct Note {
    id: Option<NoteId>, // None if not yet saved to Anki
    model: Model,
    field_values: HashMap<String, String>,
    tags: HashSet<String>,
    media: Vec<FieldMedia>,
}

impl Note {
    /// Creates a new note with validation
    pub fn new(
        model: Model,
        field_values: HashMap<String, String>,
        tags: HashSet<String>,
        media: Vec<FieldMedia>,
    ) -> std::result::Result<Self, NoteError> {
        // Check that all provided fields actually exist in the model
        for field_name in field_values.keys() {
            if model.get_field(field_name).is_none() {
                return Err(NoteError::UnknownField(field_name.to_string()));
            }
        }

        // Validate media is attached to existing fields
        for field_media in &media {
            if model.get_field(&field_media.field).is_none() {
                return Err(NoteError::UnknownField(field_media.field.clone()));
            }
        }

        Ok(Self {
            id: None,
            model,
            field_values,
            tags,
            media,
        })
    }

    /// Creates a note with an existing ID (for notes retrieved from Anki)
    pub fn with_id(
        id: NoteId,
        model: Model,
        field_values: HashMap<String, String>,
        tags: HashSet<String>,
        media: Vec<FieldMedia>,
    ) -> std::result::Result<Self, NoteError> {
        let mut note = Self::new(model, field_values, tags, media)?;
        note.id = Some(id);
        Ok(note)
    }

    /// Gets the ID of this note, if it has one
    pub fn id(&self) -> Option<NoteId> {
        self.id
    }

    /// Gets the model (note type) of this note
    pub fn model(&self) -> &Model {
        &self.model
    }

    /// Gets the field values of this note
    pub fn field_values(&self) -> &HashMap<String, String> {
        &self.field_values
    }

    /// Gets a specific field value
    pub fn field_value(&self, field_name: &str) -> Option<&String> {
        self.field_values.get(field_name)
    }

    /// Gets all tags on this note
    pub fn tags(&self) -> &HashSet<String> {
        &self.tags
    }

    /// Gets media attached to this note
    pub fn media(&self) -> &[FieldMedia] {
        &self.media
    }

    /// Returns true if this note has the given tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }

    /// Updates a field value
    pub fn update_field(
        &mut self,
        field_name: &str,
        value: String,
    ) -> std::result::Result<(), NoteError> {
        if self.model.get_field(field_name).is_none() {
            return Err(NoteError::UnknownField(field_name.to_string()));
        }
        self.field_values.insert(field_name.to_string(), value);
        Ok(())
    }

    /// Adds a tag
    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }

    /// Removes a tag
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        self.tags.remove(tag)
    }

    /// Adds media to a field
    pub fn add_media(
        &mut self,
        field_name: &str,
        media: Media,
    ) -> std::result::Result<(), NoteError> {
        if self.model.get_field(field_name).is_none() {
            return Err(NoteError::UnknownField(field_name.to_string()));
        }

        self.media.push(FieldMedia {
            media,
            field: field_name.to_string(),
        });

        Ok(())
    }

    /// Gets the value for the front (question) field
    pub fn front_value(&self) -> Option<&String> {
        self.model
            .front_field()
            .and_then(|field| self.field_values.get(field.name()))
    }

    /// Gets the value for the back (answer) field
    pub fn back_value(&self) -> Option<&String> {
        self.model
            .back_field()
            .and_then(|field| self.field_values.get(field.name()))
    }
}
