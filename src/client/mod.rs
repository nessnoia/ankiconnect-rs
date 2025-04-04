//! Client interfaces for interacting with Anki through AnkiConnect
//!
//! This module provides domain-specific clients for different aspects of
//! the Anki application - cards, decks, media, and models.

// Declare submodules
mod anki_client;
mod cards;
mod decks;
mod media;
mod models;
pub mod request;

pub use anki_client::AnkiClient;
pub use self::cards::DuplicateScope;

// Re-export domain-specific clients
pub(crate) use self::cards::CardClient;
pub(crate) use self::decks::DeckClient;
pub(crate) use self::media::MediaClient;
pub(crate) use self::models::ModelClient;
