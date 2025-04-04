use ankiconnect_rs::{AnkiClient, Model, Result};

fn main() -> Result<()> {
    // Create a client with default connection (localhost:8765)
    println!("Connecting to Anki...");
    let client = AnkiClient::new();

    // Verify connection by checking version
    let version = client.version()?;
    println!("Connected to AnkiConnect version: {}", version);

    // Get all available models
    println!("\nFetching all models...");
    let models = client.models().get_all()?;

    println!("\nFound {} models:", models.len());

    // Display each model and its fields
    for (i, model) in models.iter().enumerate() {
        print_model_details(i + 1, model);
    }

    Ok(())
}

fn print_model_details(index: usize, model: &Model) {
    println!(
        "\n{}. Model: {} (ID: {})",
        index,
        model.name(),
        model.id().0
    );

    // Print field information
    println!("   Fields ({}):", model.fields().len());
    for field in model.fields() {
        println!("   - {} (position: {})", field.name(), field.ord());

        // Add some helpful info about likely roles
        if field.is_front() {
            println!("     Likely role: Question/Front field");
        } else if field.is_back() {
            println!("     Likely role: Answer/Back field");
        }
    }
}
