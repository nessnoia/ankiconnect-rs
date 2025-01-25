[![Crates.io](https://img.shields.io/crates/v/ankiconnect-rs.svg)](https://crates.io/crates/ankiconnect-rs)
[![Documentation](https://docs.rs/ankiconnect-rs/badge.svg)](https://docs.rs/ankiconnect-rs/)
[![Codecov](https://codecov.io/github/btrkeks/ankiconnect-rs/coverage.svg?branch=master)](https://codecov.io/gh/btrkeks/ankiconnect-rs)
[![Dependency status](https://deps.rs/repo/github/btrkeks/ankiconnect-rs/status.svg)](https://deps.rs/repo/github/btrkeks/ankiconnect-rs)

# ankiconnect-rs

A work-in-progress Rust crate for interacting with [AnkiConnect](https://foosoft.net/projects/anki-connect/),
enabling convenient programmatic control of Anki from within Rust.
Provides type-safe abstractions for common Anki operations.

## Features

- 🃏 **Card Management**: Create notes, find cards, browse cards via GUI  
- 🗃️ **Deck Operations**: Create decks, list existing decks  
- 📦 **Media Handling**: Store media files from paths/URLs/base64 data  
- 🧩 **Model Support**: Fetch field names, validate note structures  
- 🔄 **Error Handling**: Comprehensive error types for AnkiConnect-specific issues  
- ✅ **Tested**: Mock server integration tests for all major operations  

## Prerequisites

1. [Anki](https://apps.ankiweb.net/) with [AnkiConnect](https://foosoft.net/projects/anki-connect/) installed  
2. Anki running with AnkiConnect enabled (default: `localhost:8765`)

## Usage

### Basic Example
Note: The example is not tested, so slight adjustments might be necessary.
```rust
use anki_connect_rs::{
    AnkiClient,
    anki_card::AnkiCardBuilder,
    error::{AnkiRequestError, AnkiConnectError}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use default url 'localhost' and port '8765'
    let client = AnkiClient::default();

    // Get existing decks
    let decks: Vec<AnkiDeck> = client.get_all_decks()?;

    // Get available note types
    let models: Vec<AnkiModelIdentifier> = client.get_all_models()?;

    // Create a note using fetched resources
    let card = AnkiCardBuilder::new_for_model(models.get(0)?.clone())
        .add_field("Front", "¿Dónde está la biblioteca?")
        .add_field("Back", "Where is the library?")
        .add_tag("spanish-vocab")
        .build();

    // Add note to Anki
    match client.add_note(decks.get(0)?.clone(), card, false, None) {
        Ok(note_id) => println!("Added note with ID: {}", note_id),
        Err(AnkiRequestError::AnkiConnectError(AnkiConnectError::DeckNotFound(_))) => {
            eprintln!("Create the Spanish deck in Anki first!");
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
```