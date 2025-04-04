//! Domain models for Anki entities
//!
//! This module contains type-safe representations of Anki's core concepts
//! such as decks, cards, note types, and fields.

// Declare submodules
mod card;
mod deck;
mod media;
mod model;
mod note;

// Re-export primary types
pub use self::card::{Card, CardId};
pub use self::deck::{Deck, DeckConfig, DeckId, DeckStats};
pub use self::media::{FieldMedia, Media, MediaSource, MediaType};
pub use self::model::{Field, FieldRef, Model, ModelId};
pub use self::note::{Note, NoteId};
