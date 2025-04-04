//! Client for Anki deck operations

use super::request::{self, CreateDeckParams, DeckConfigsResult, DeckTreeNode};
use crate::error::{AnkiError, Result};
use crate::http::{HttpRequestSender, RequestSender};
use crate::models::{CardId, Deck, DeckConfig, DeckId, DeckStats};
use crate::QueryBuilder;
use std::collections::HashMap;
use std::sync::Arc;

/// Client for deck-related operations
pub struct DeckClient {
    sender: Arc<HttpRequestSender>,
}

impl DeckClient {
    /// Creates a new DeckClient with the given request sender
    pub(crate) fn new(sender: Arc<HttpRequestSender>) -> Self {
        Self { sender }
    }

    /// Gets all decks from Anki
    ///
    /// # Returns
    ///
    /// A list of all decks in the Anki collection
    pub fn get_all(&self) -> Result<Vec<Deck>> {
        let result: HashMap<std::string::String, u64> =
            self.sender.send("deckNamesAndIds", None::<()>)?;

        Ok(result
            .into_iter()
            .map(|(name, id)| Deck::new(id, name))
            .collect())
    }

    /// Gets a deck by its name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the deck to get
    ///
    /// # Returns
    ///
    /// The deck with the given name, if it exists
    pub fn get_by_name(&self, name: &str) -> Result<Option<Deck>> {
        let decks = self.get_all()?;
        Ok(decks.into_iter().find(|d| d.name() == name))
    }

    /// Gets a deck by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the deck to get
    ///
    /// # Returns
    ///
    /// The deck with the given ID, if it exists
    pub fn get_by_id(&self, id: DeckId) -> Result<Option<Deck>> {
        let decks = self.get_all()?;
        Ok(decks.into_iter().find(|d| d.id() == id))
    }

    /// Creates a new deck
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the deck to create
    ///
    /// # Returns
    ///
    /// The ID of the created deck
    pub fn create(&self, name: &str) -> Result<DeckId> {
        if name.is_empty() {
            return Err(AnkiError::ValidationError(
                "Deck name cannot be empty".to_string(),
            ));
        }

        let params = CreateDeckParams { deck: name };
        let id = self.sender.send::<_, u64>("createDeck", Some(params))?;

        Ok(DeckId(id))
    }

    /// Deletes a deck
    ///
    /// # Arguments
    ///
    /// * `deck_id` - The ID of the deck to delete
    /// * `cards_too` - Whether to delete the cards in the deck as well
    pub fn delete(&self, deck_name: &str, cards_too: bool) -> Result<()> {
        let params = request::DeleteDeckParams {
            decks: &[deck_name],
            cards_too,
        };

        self.sender.send::<_, ()>("deleteDecks", Some(params))
    }

    /// Gets the deck configurations (options groups)
    ///
    /// # Returns
    ///
    /// A list of deck configurations
    pub fn get_configurations(&self) -> Result<Vec<DeckConfig>> {
        let result: DeckConfigsResult = self.sender.send("getDeckConfig", None::<()>)?;

        Ok(result
            .config_list
            .into_iter()
            .map(DeckConfig::from)
            .collect())
    }

    /// Gets the deck tree structure
    ///
    /// # Returns
    ///
    /// The hierarchical deck tree
    pub fn get_tree(&self) -> Result<Vec<DeckTreeNode>> {
        self.sender.send("deckTree", None::<()>)
    }

    /// Gets statistics for a single deck
    ///
    /// # Arguments
    ///
    /// * `deck_name` - The name of the deck to get statistics for
    ///
    /// # Returns
    ///
    /// Statistics for the deck
    pub fn get_stat(&self, deck_name: &str) -> Result<DeckStats> {
        // There is no API call for a single deck
        let stats_map = self.get_stats(&[deck_name])?;

        // TODO: What do if there are multiple?
        let (_, stats) = stats_map
            .into_iter()
            .next()
            .ok_or_else(|| AnkiError::UnknownError("No stats found for deck".to_string()))?;

        // Convert the DTO to the domain model using the From implementation
        Ok(stats)
    }

    /// Gets statistics for multiple decks
    ///
    /// # Arguments
    ///
    /// * `deck_names` - The names of the decks to get statistics for
    ///
    /// # Returns
    ///
    /// A Hashmap mapping the ids of the decks to their statistics
    pub fn get_stats(&self, deck_names: &[&str]) -> Result<HashMap<String, DeckStats>> {
        let params = request::DeckStatsParams { decks: deck_names };

        // Deserialize into the DTO first, then convert
        let stats_dto_map: HashMap<String, request::DeckStatsDto> =
            self.sender.send("getDeckStats", Some(params))?;

        Ok(stats_dto_map
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect())
    }

    /// Gets all cards in a deck
    ///
    /// # Arguments
    ///
    /// * `deck_id` - The ID of the deck to get cards from
    ///
    /// # Returns
    ///
    /// A list of card IDs in the deck
    pub fn get_cards_in_deck(&self, deck_name: &str) -> Result<Vec<CardId>> {
        let query = QueryBuilder::new().in_deck(deck_name).build();
        let params = request::FindCardsParams {
            query: query.as_str(),
        };
        let ids = self.sender.send::<_, Vec<u64>>("findCards", Some(params))?;
        Ok(ids.into_iter().map(CardId).collect())
    }

    /// Checks if a deck with the given name exists
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the deck to check
    ///
    /// # Returns
    ///
    /// `true` if the deck exists, `false` otherwise
    pub fn exists(&self, name: &str) -> Result<bool> {
        let decks = self.get_all()?;
        Ok(decks.into_iter().any(|d| d.name() == name))
    }
}
