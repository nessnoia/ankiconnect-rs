//! Card model definitions

use crate::models::{Field, MediaSource, MediaType, Model};
use std::collections::{HashMap, HashSet};

/// Unique identifier for an Anki card
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardId(pub u64);

impl CardId {
    /// Gets the raw ID value
    pub fn value(&self) -> u64 {
        self.0
    }
}

/// Represents a complete Anki card with all its data
#[derive(Debug, Clone)]
pub struct Card {
    model: Model,
    field_values: HashMap<String, String>,
    tags: HashSet<String>,
    media: Vec<(MediaType, MediaSource, String, Field)>,
}

impl Card {
    /// Creates a new card with the given model and data
    pub(crate) fn new(
        model: Model,
        field_values: HashMap<String, String>,
        tags: HashSet<String>,
        media: Vec<(MediaType, MediaSource, String, Field)>,
    ) -> Self {
        Self {
            model,
            field_values,
            tags,
            media,
        }
    }

    /// Gets the model (note type) of this card
    pub fn model(&self) -> &Model {
        &self.model
    }

    /// Gets the field values of this card
    pub fn fields(&self) -> impl Iterator<Item = (&Field, &String)> {
        // Map field names to the actual Field objects
        self.field_values.iter().filter_map(|(name, value)| {
            // Find the field object by name
            self.model.get_field(name).map(|field| (field, value))
        })
    }

    /// Gets a specific field value
    pub fn field_value(&self, field_name: &str) -> Option<&String> {
        self.field_values.get(field_name)
    }

    /// Gets all tags on this card
    pub fn tags(&self) -> impl Iterator<Item = &String> {
        self.tags.iter()
    }

    /// Gets media attached to this card
    pub fn media(&self) -> impl Iterator<Item = &(MediaType, MediaSource, String, Field)> {
        self.media.iter()
    }

    /// Returns true if this card has the given tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
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
