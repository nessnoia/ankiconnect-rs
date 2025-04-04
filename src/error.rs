//! Error types for the ankiconnect-rs crate

use thiserror::Error;

pub type Result<T> = std::result::Result<T, AnkiError>;

/// Main error type for the ankiconnect-rs crate
#[derive(Error, Debug)]
pub enum AnkiError {
    /// Error from the AnkiConnect API
    #[error(transparent)]
    AnkiConnectError(#[from] AnkiConnectError),

    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] ureq::Error),

    /// JSON parsing error
    #[error("JSON parsing failed: {0}")]
    JsonError(String),

    /// Invalid field for the given model
    #[error("Invalid field '{field_name}' for model '{model_name}'")]
    InvalidField {
        field_name: String,
        model_name: String,
    },

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}

/// Errors specific to the AnkiConnect API
#[derive(Error, Debug)]
pub enum AnkiConnectError {
    /// The action is not supported by AnkiConnect
    #[error("Unsupported action")]
    UnsupportedAction,

    /// Field values would make an empty question
    #[error("The field values would make an empty question")]
    EmptyQuestion,

    /// The specified model was not found
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    /// The specified deck was not found
    #[error("Deck not found: {0}")]
    DeckNotFound(String),

    /// The note is a duplicate
    #[error("Duplicate note")]
    DuplicateNote,

    /// The note is empty
    #[error("Empty note")]
    EmptyNote,

    /// Missing media field (data, path, or URL)
    #[error("Missing media field (provide data, path, or URL)")]
    MissingMediaField,

    /// A model with this name already exists
    #[error("Model name already exists")]
    ModelNameExists,

    /// Invalid column ID
    #[error("Invalid column ID: {0}")]
    InvalidColumnId(String),

    /// Invalid card order
    #[error("Invalid card order: {0}")]
    InvalidCardOrder(String),

    /// Other unspecified AnkiConnect error
    #[error("Other error: {0}")]
    Other(String),
}

/// Errors that can occur when creating or manipulating notes
#[derive(Error, Debug)]
pub enum NoteError {
    /// A required field was missing from the note
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// An unknown field was provided
    #[error("Unknown field: {0}")]
    UnknownField(String),

    /// A field was provided with empty content
    #[error("Empty content for field: {0}")]
    EmptyField(String),

    /// Field content would make an empty question
    #[error("The provided field content would result in an empty question")]
    EmptyQuestion,

    /// Media file is missing or invalid
    #[error("Invalid media: {0}")]
    InvalidMedia(String),

    /// The note would be a duplicate
    #[error("Note would be a duplicate of an existing note")]
    DuplicateNote,

    /// Other validation error
    #[error("Note validation error: {0}")]
    ValidationError(String),

    /// Field content validation error
    #[error("Field content validation error for '{field}': {message}")]
    FieldValidationError { field: String, message: String },

    /// Conversion from AnkiError
    #[error("Anki error: {0}")]
    AnkiError(#[from] AnkiError),
}

// Implement conversion from AnkiConnectError to NoteError for convenience
impl From<AnkiConnectError> for NoteError {
    fn from(err: AnkiConnectError) -> Self {
        use crate::error::AnkiConnectError;
        match err {
            AnkiConnectError::EmptyQuestion => NoteError::EmptyQuestion,
            AnkiConnectError::DuplicateNote => NoteError::DuplicateNote,
            AnkiConnectError::EmptyNote => NoteError::ValidationError("Note is empty".to_string()),
            e => NoteError::AnkiError(e.into()),
        }
    }
}
