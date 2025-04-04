//! Builders for creating complex Anki objects
//!
//! This module provides fluent builder interfaces for constructing
//! well-formed Anki objects like cards and search queries.

// Declare submodules
mod note;
mod query;

// Re-export public builders
pub use self::note::NoteBuilder;
pub use self::query::{CardState, Flag, Query, QueryBuilder};
