use ankiconnect_rs::{AnkiClient, MediaSource, NoteBuilder};
use anyhow::anyhow;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    // Create a client with default connection (localhost:8765)
    println!("Connecting to Anki...");
    let client = AnkiClient::new();

    // Verify connection by checking version
    let version = client.version()?;
    println!("Connected to AnkiConnect version: {}", version);

    // Get all available decks and prompt user to select one
    println!("\nFetching all decks...");
    let decks = client.decks().get_all()?;

    println!("\nSelect a deck by entering its number:");
    for (i, deck) in decks.iter().enumerate() {
        println!("{}. {}", i + 1, deck.name());
    }

    let deck_index = read_user_choice(decks.len())?;
    let selected_deck = &decks[deck_index - 1];
    println!("Selected deck: {}", selected_deck.name());

    // Get all available models and prompt user to select one
    println!("\nFetching all models (note types)...");
    let models = client.models().get_all()?;

    println!("\nSelect a note type by entering its number:");
    for (i, model) in models.iter().enumerate() {
        println!("{}. {}", i + 1, model.name());
    }

    let model_index = read_user_choice(models.len())?;
    let selected_model = &models[model_index - 1];
    println!("Selected note type: {}", selected_model.name());

    // Get field information for the selected model
    println!("\nFields in the selected note type:");
    for field in selected_model.fields() {
        println!("- {}", field.name());
    }

    // Determine front and back fields based on common naming conventions
    let front_field = selected_model.front_field();
    let back_field = selected_model.back_field();

    if front_field.is_none() {
        println!("Could not determine a front field for the selected model.");
        return Ok(());
    }

    // Build the note using NoteBuilder
    let front_field = front_field.unwrap();
    println!("\nBuilding note...");
    let mut builder = NoteBuilder::new(selected_model.clone())
        .with_field(front_field, "Dog")
        .with_tag("example")
        .with_tag("animal");

    // Add back field content if available
    if let Some(back_field) = back_field {
        builder = builder
            .with_field(back_field, "A friendly animal that makes a good pet.")
            .with_image(
                back_field,
                MediaSource::Url(
                    "https://cdn.pixabay.com/photo/2023/08/18/15/02/dog-8198719_640.jpg"
                        .to_string(),
                ),
                "example_dog.jpg",
            );
        println!("Added content to the back field: {}", back_field.name());
    } else {
        println!("Could not determine a back field for the selected model.");
    }

    // Build the note
    let note = builder.build()?;

    // Add the note to Anki
    println!("\nAdding note to Anki...");
    let note_id = client.cards().add_note(selected_deck, note, false, None)?;

    println!("\nSuccess! Note added with ID: {}", note_id.value());
    println!("You should now see a new card with a dog image in your Anki deck.");

    Ok(())
}

// Helper function to read and validate user input for selection
fn read_user_choice(max_value: usize) -> anyhow::Result<usize> {
    let mut input = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    let index = input.trim().parse::<usize>().map_err(|_| {
        ankiconnect_rs::AnkiError::ValidationError("Please enter a valid number".to_string())
    })?;

    if index < 1 || index > max_value {
        return Err(anyhow!("Please enter a number between 1 and {}", max_value));
    }

    Ok(index)
}
