//! # ankiconnect-rs
//!
//! A Rust crate for interacting with [AnkiConnect](https://foosoft.net/projects/anki-connect/),
//! enabling convenient programmatic control of Anki from within Rust.
//! Provides type-safe abstractions for common Anki operations.
//!
//! ## Features
//!
//! - 🃏 **Card Management**: Create notes, find cards, browse cards via GUI  
//! - 🗃️ **Deck Operations**: Create decks, list existing decks  
//! - 📦 **Media Handling**: Store media files from paths/URLs/base64 data  
//! - 🧩 **Model Support**: Fetch field names, validate note structures  
//! - 🔄 **Error Handling**: Comprehensive error types for AnkiConnect-specific issues  
//! - ✅ **Tested**: Mock server integration tests for all major operations
//!
//! ## Example
//!
//! ```rust,no_run
//! use ankiconnect_rs::{AnkiClient, NoteBuilder, Field, MediaSource};
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     // Create a client with default connection (localhost:8765)
//!     let client = AnkiClient::new();
//!
//!     // Get available decks and models
//!     let decks = client.decks().get_all()?;
//!     let models = client.models().get_all()?;
//!
//!     // Get fields for the selected model
//!     let selected_model = &models[0];
//!     let fields = client.models().get_fields(selected_model)?;
//!
//!     // Build a note with the selected model
//!     let front_field = selected_model.field_ref("Front").unwrap();
//!     let back_field = selected_model.field_ref("Back").unwrap();
//!
//!     let note = NoteBuilder::new(selected_model.clone())
//!         .with_field(front_field, "¿Dónde está la biblioteca?")
//!         .with_field(back_field, "Where is the library?")
//!         .with_tag("spanish-vocab")
//!         .with_image(
//!             front_field,
//!             MediaSource::Url("https://cdn.pixabay.com/photo/2023/08/18/15/02/dog-8198719_640.jpg".to_string()),
//!             "test_dog.jpg"
//!         )
//!         .build()?;
//!
//!     // Add the note to the first deck
//!     let note_id = client.cards().add_note(&decks[0], note, false, None)?;
//!     println!("Added note with ID: {}", note_id.value());
//!
//!     Ok(())
//! }
//! ```

// Re-export key types for a clean public API
pub use builders::{NoteBuilder, QueryBuilder};
pub use client::{AnkiClient, DuplicateScope};
pub use error::{AnkiConnectError, AnkiError, NoteError, Result};
pub use models::{
    Card, CardId, Deck, DeckId, Field, FieldMedia, Media, MediaSource, MediaType, Model, Note,
    NoteId,
};

// Public modules
pub mod builders;
pub mod client;
pub mod error;
pub mod models;

// Private modules
mod http;
