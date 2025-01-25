use thiserror::Error;
use crate::anki_card::{AnkiModelIdentifier};

#[derive(Error, Debug)]
pub enum AnkiConnectError {
    #[error("Unsupported action")]
    UnsupportedAction, // should never happen if there are no bugs in here
    #[error("The field values would make an empty question")]
    EmptyQuestion,
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    #[error("Deck not found: {0}")]
    DeckNotFound(String),
    #[error("Duplicate note")]
    DuplicateNote,
    #[error("Empty note")]
    EmptyNote,
    #[error("Missing media field (provide data, path, or url)")]
    MissingMediaField, // TODO: should also not happen, though need testing for empty data / path / url and especially non-existing files, url, etc.
    #[error("Model name already exists")]
    ModelNameExists,
    #[error("Invalid column ID: {0}")]
    InvalidColumnId(String),
    #[error("Invalid card order: {0}")]
    InvalidCardOrder(String),
    #[error("Other error: {0}")]
    Other(String),
}

pub(crate) fn parse_anki_connect_error(error: &str) -> AnkiConnectError {
    if error.starts_with("deck was not found: ") {
        let deck_name = error.trim_start_matches("deck was not found: ").trim();
        AnkiConnectError::DeckNotFound(deck_name.to_string())
    } else if error.starts_with("model was not found: ") {
        let model_name = error.trim_start_matches("model was not found: ").trim();
        AnkiConnectError::ModelNotFound(model_name.to_string())
    } else if error == "cannot create note because it is a duplicate" {
        AnkiConnectError::DuplicateNote
    } else if error == "cannot create note because it is empty" {
        AnkiConnectError::EmptyNote
    } else if error.starts_with("invalid columnId: ") {
        let column_id = error.trim_start_matches("invalid columnId: ").trim();
        AnkiConnectError::InvalidColumnId(column_id.to_string())
    } else if error.starts_with("invalid card order: ") {
        let order = error.trim_start_matches("invalid card order: ").trim();
        AnkiConnectError::InvalidCardOrder(order.to_string())
    } else if error == "You must provide a \"data\", \"path\", or \"url\" field." {
        AnkiConnectError::MissingMediaField
    } else if error == "Model name already exists" {
        AnkiConnectError::ModelNameExists
    } else if error
        == "The field values you have provided would make an empty question on all cards."
    {
        AnkiConnectError::EmptyQuestion
    } else if error == "unsupported action" {
        AnkiConnectError::UnsupportedAction
    } else {
        AnkiConnectError::Other(error.to_string())
    }
}

#[derive(Error, Debug)]
pub enum AnkiRequestError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] ureq::Error),
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] std::io::Error),
    #[error(transparent)]
    AnkiConnectError(#[from] AnkiConnectError),
    #[error("Invalid field: {0} for model {1}")]
    InvalidField(String, AnkiModelIdentifier),
    #[error("Validation error: {0}")]
    ValidationError(String),
}