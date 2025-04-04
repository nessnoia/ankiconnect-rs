//! Deck model definitions

use crate::client::request::DeckStatsDto;

/// Unique identifier for a deck
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeckId(pub u64);

/// Represents an Anki deck
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck {
    id: DeckId,
    name: String,
}

impl Deck {
    /// Creates a new deck with the given ID and name
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id: DeckId(id),
            name,
        }
    }

    /// Gets the ID of this deck
    pub fn id(&self) -> DeckId {
        self.id
    }

    /// Gets the name of this deck
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Checks if this deck is a subdeck
    pub fn is_subdeck(&self) -> bool {
        self.name.contains("::")
    }

    /// Gets the parent deck name, if this is a subdeck
    pub fn parent_name(&self) -> Option<&str> {
        if self.is_subdeck() {
            self.name.rsplit_once("::").map(|(parent, _)| parent)
        } else {
            None
        }
    }

    /// Gets the immediate name of this deck (without parent hierarchy)
    pub fn base_name(&self) -> &str {
        if self.is_subdeck() {
            self.name.rsplit_once("::").map(|(_, base)| base).unwrap()
        } else {
            &self.name
        }
    }
}

/// Represents deck configuration options
#[derive(Debug, Clone)]
pub struct DeckConfig {
    pub id: u64,
    pub name: String,
    pub reuse_if_possible: bool,
    pub disable_auto_qe: bool,
}

impl From<crate::client::request::DeckConfigDto> for DeckConfig {
    fn from(dto: crate::client::request::DeckConfigDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            reuse_if_possible: dto.reuse_if_possible,
            disable_auto_qe: dto.disable_auto_qe,
        }
    }
}

/// Statistics for a deck
#[derive(Debug, Clone)]
pub struct DeckStats {
    pub deck_id: u64,
    pub new_count: u32,
    pub learn_count: u32,
    pub review_count: u32,
    pub total_in_deck: u32,
}

impl From<crate::client::request::DeckStatsDto> for DeckStats {
    fn from(dto: DeckStatsDto) -> Self {
        Self {
            deck_id: dto.deck_id,
            new_count: dto.new_count,
            learn_count: dto.learn_count,
            review_count: dto.review_count,
            total_in_deck: dto.total_in_deck,
        }
    }
}
