[![Crates.io](https://img.shields.io/crates/v/ankiconnect-rs.svg)](https://crates.io/crates/ankiconnect-rs)
[![Documentation](https://docs.rs/ankiconnect-rs/badge.svg)](https://docs.rs/ankiconnect-rs/)
[![Codecov](https://codecov.io/github/btrkeks/ankiconnect-rs/coverage.svg?branch=master)](https://codecov.io/gh/btrkeks/ankiconnect-rs)
[![Dependency status](https://deps.rs/repo/github/btrkeks/ankiconnect-rs/status.svg)](https://deps.rs/repo/github/btrkeks/ankiconnect-rs)

# ankiconnect-rs

A work-in-progress Rust crate for interacting with [AnkiConnect](https://foosoft.net/projects/anki-connect/),
enabling convenient programmatic control of Anki from within Rust applications.
Provides type-safe abstractions for common Anki operations with a clean domain-driven API.

## Features

- 🃏 **Card Management**: Create notes, find cards, browse cards via the Anki GUI
- 🗃️ **Deck Operations**: Create decks, list existing decks, get statistics
- 📦 **Media Handling**: Store media files from paths/URLs/base64 data
- 🧩 **Model Support**: Work with note types, validate fields, manage templates
- 🔍 **Search Capabilities**: Build complex search queries with a fluent interface
- 🔄 **Error Handling**: Comprehensive error types for AnkiConnect-specific issues
- ✅ **Well Tested**: Mock server integration tests for all major operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ankiconnect-rs = "1.0.0"
```

## Prerequisites

1. [Anki](https://apps.ankiweb.net/) with [AnkiConnect](https://foosoft.net/projects/anki-connect/) installed
2. Anki running with AnkiConnect enabled (default: `localhost:8765`)

## API Overview

The crate is organized around a central `AnkiClient` that provides access to domain-specific clients:

- **`client.cards()`** - Operations for notes and cards (add notes, find cards, etc.)
- **`client.decks()`** - Operations for decks (create, list, get stats, etc.)
- **`client.models()`** - Operations for note types (get fields, templates, etc.)
- **`client.media()`** - Operations for media files (store, retrieve, etc.)

## Usage

### Basic Example

```rust
use ankiconnect_rs::{AnkiClient, DuplicateScope, NoteBuilder};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a client with default connection (localhost:8765)
    let client = AnkiClient::new();

    // Get available decks and models
    let decks = client.decks().get_all()?;
    let models = client.models().get_all()?;

    // Build a note with the selected model
    let selected_model = &models[0];
    let front_field = selected_model.field_ref("Front").unwrap();
    let back_field = selected_model.field_ref("Back").unwrap();

    let note = NoteBuilder::new(selected_model.clone())
        .with_field(front_field, "¿Dónde está la biblioteca?")
        .with_field(back_field, "Where is the library?")
        .with_tag("spanish-vocab")
        .build()?;

    // Add the note to the first deck
    let note_id = client.cards().add_note(&decks[0], note, false, None)?;
    println!("Added note with ID: {}", note_id.value());

    Ok(())
}
```

### Adding Media to Notes

```rust
use ankiconnect_rs::{AnkiClient, MediaSource, NoteBuilder};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = AnkiClient::new();
    let decks = client.decks().get_all()?;
    let models = client.models().get_all()?;
    
    let selected_model = &models[0];
    let front_field = selected_model.field_ref("Front").unwrap();
    let back_field = selected_model.field_ref("Back").unwrap();

    let note = NoteBuilder::new(selected_model.clone())
        .with_field(front_field, "Dog")
        .with_field(back_field, "A friendly animal")
        .with_tag("animals")
        // Add an image to the front field
        .with_image(
            front_field,
            MediaSource::Url("https://example.com/dog.jpg".to_string()),
            "dog.jpg"
        )
        .build()?;

    client.cards().add_note(&decks[0], note, false, None)?;
    Ok(())
}
```

### Building Search Queries

```rust
use ankiconnect_rs::{AnkiClient, QueryBuilder, CardState};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = AnkiClient::new();
    
    // Build a complex search query
    let query = QueryBuilder::new()
        .in_deck("Japanese::Vocabulary")
        .and()
        .field("Front").contains("犬")
        .and()
        .not()
        .in_state(CardState::Suspended)
        .build();
    
    // Find cards matching the query
    let cards = client.cards().find(&query)?;
    println!("Found {} matching cards", cards.len());
    
    Ok(())
}
```

## More Examples

See the [examples directory](https://github.com/btrkeks/ankiconnect-rs/tree/master/examples) for more complete examples:

- `list_decks.rs` - Listing decks and their information
- `list_models.rs` - Listing models (note types) and their fields
- `add_note.rs` - Interactive example of adding a note with media

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.
