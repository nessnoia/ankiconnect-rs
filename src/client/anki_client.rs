use crate::client::{CardClient, DeckClient, MediaClient, ModelClient};
use crate::http::HttpRequestSender;
use crate::AnkiError;
use std::sync::Arc;

/// The main client for interacting with Anki via AnkiConnect
///
/// This is the primary entry point for the library. It provides access to specialized
/// clients for different aspects of Anki functionality.
pub struct AnkiClient {
    cards_client: CardClient,
    decks_client: DeckClient,
    media_client: MediaClient,
    models_client: ModelClient,
}

impl AnkiClient {
    /// Creates a new client with the default connection (localhost:8765)
    pub fn new() -> Self {
        Self::with_connection("localhost", 8765)
    }

    /// Creates a new client with a custom host and port
    pub fn with_connection(host: &str, port: u16) -> Self {
        let sender = Arc::new(HttpRequestSender::new(host, port));
        Self {
            cards_client: CardClient::new(Arc::clone(&sender)),
            decks_client: DeckClient::new(Arc::clone(&sender)),
            media_client: MediaClient::new(Arc::clone(&sender)),
            models_client: ModelClient::new(sender),
        }
    }

    /// Gets the version of the AnkiConnect plugin
    pub fn version(&self) -> Result<u16, AnkiError> {
        self.cards_client.get_version()
    }

    /// Access operations related to cards and notes
    pub fn cards(&self) -> &CardClient {
        &self.cards_client
    }

    /// Access operations related to decks
    pub fn decks(&self) -> &DeckClient {
        &self.decks_client
    }

    /// Access operations related to media files
    pub fn media(&self) -> &MediaClient {
        &self.media_client
    }

    /// Access operations related to note types (models)
    pub fn models(&self) -> &ModelClient {
        &self.models_client
    }
}

impl Default for AnkiClient {
    fn default() -> Self {
        Self::new()
    }
}
