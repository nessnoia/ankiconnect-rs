use crate::error::{AnkiError, Result};
use std::collections::HashSet;
use thiserror::Error;

/// Unique identifier for an Anki model (note type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModelId(pub u64);

/// Represents a field within a model
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {
    name: String,
    ord: usize, // Field ordinal/position in the model
}

impl Field {
    /// Creates a new field with the given name and ordinal
    pub fn new(name: String, ord: usize) -> Self {
        Self { name, ord }
    }

    /// Gets the name of this field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the ordinal (position) of this field in its model
    pub fn ord(&self) -> usize {
        self.ord
    }

    /// Returns true if this is likely a "Front" field
    pub fn is_front(&self) -> bool {
        self.name.eq_ignore_ascii_case("front")
            || self.name.contains("front")
            || self.name.contains("question")
    }

    /// Returns true if this is likely a "Back" field
    pub fn is_back(&self) -> bool {
        self.name.eq_ignore_ascii_case("back")
            || self.name.contains("back")
            || self.name.contains("answer")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Model {
    id: ModelId,
    name: String,
    fields: Vec<Field>,
}

impl Model {
    /// Creates a new model with validation
    pub fn new(id: u64, name: String, fields: Vec<Field>) -> Result<Self> {
        // Ensure the model has at least one field
        if fields.is_empty() {
            return Err(AnkiError::ValidationError(
                "Model must have at least one field".to_string(),
            ));
        }

        // Check for duplicate field names
        let mut seen_names = HashSet::new();
        for field in &fields {
            if !seen_names.insert(field.name()) {
                return Err(AnkiError::ValidationError(format!(
                    "Duplicate field name: {}",
                    field.name()
                )));
            }
        }

        Ok(Self {
            id: ModelId(id),
            name,
            fields,
        })
    }

    /// Gets the ID of this model
    pub fn id(&self) -> ModelId {
        self.id
    }

    /// Gets the name of this model
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets all fields in this model
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    /// Find a field by name
    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name() == name)
    }

    /// Get a strongly-typed reference to a field
    pub fn field_ref(&self, name: &str) -> Option<FieldRef<'_>> {
        self.get_field(name)
            .map(|field| FieldRef { model: self, field })
    }

    /// Gets the "front" field if it can be determined
    pub fn front_field(&self) -> Option<&Field> {
        self.fields.iter().find(|f| f.is_front())
    }

    /// Gets the "back" field if it can be determined
    pub fn back_field(&self) -> Option<&Field> {
        self.fields.iter().find(|f| f.is_back())
    }
}

/// A type-safe reference to a field in a model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldRef<'a> {
    model: &'a Model,
    field: &'a Field,
}

impl<'a> FieldRef<'a> {
    /// Gets the name of this field
    pub fn name(&self) -> &str {
        self.field.name()
    }

    /// Gets the model this field belongs to
    pub fn model(&self) -> &'a Model {
        self.model
    }

    /// Gets the underlying Field
    pub fn field(&self) -> &'a Field {
        self.field
    }
}

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Model must have at least one field")]
    NoFields,

    #[error("Duplicate field name: {0}")]
    DuplicateFieldName(String),
}
