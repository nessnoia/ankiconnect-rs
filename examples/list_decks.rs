use ankiconnect_rs::{AnkiClient, Deck, Result};

fn main() -> Result<()> {
    // Create a client with default connection (localhost:8765)
    println!("Connecting to Anki...");
    let client = AnkiClient::new();

    // Verify connection by checking version
    let version = client.version()?;
    println!("Connected to AnkiConnect version: {}", version);

    // Get all available decks
    println!("\nFetching all decks...");
    let decks = client.decks().get_all()?;

    println!("\nFound {} decks:", decks.len());

    // Display basic information about each deck
    for (i, deck) in decks.iter().enumerate() {
        print_deck_info(i + 1, deck);

        // Try to get deck statistics
        // Note that in a real implementation you should instead use `get_stats` (all at once)
        // instead of one by one in a loop
        match client.decks().get_stat(deck.name()) {
            Ok(stats) => {
                println!("   Statistics:");
                println!("      - New cards: {}", stats.new_count);
                println!("      - Cards in learning: {}", stats.learn_count);
                println!("      - Cards for review: {}", stats.review_count);
                println!("      - Total cards: {}", stats.total_in_deck);
            }
            Err(e) => {
                println!("   Statistics: Not available ({})", e);
            }
        }

        // Get cards in this deck
        match client.decks().get_cards_in_deck(deck.name()) {
            Ok(cards) => {
                println!("   Cards: {} cards found", cards.len());

                // Show first few card IDs as an example
                if !cards.is_empty() {
                    println!("   First {} card IDs:", cards.len().min(3));
                    for (idx, card) in cards.iter().take(3).enumerate() {
                        println!("      {}. {}", idx + 1, card.0);
                    }

                    if cards.len() > 3 {
                        println!("      ... and {} more", cards.len() - 3);
                    }
                }
            }
            Err(e) => {
                println!("   Cards: Could not retrieve ({})", e);
            }
        }

        println!(); // Add a blank line between decks
    }

    // Show deck hierarchy using deck tree if available
    println!("\nDeck Hierarchy:");
    match client.decks().get_tree() {
        Ok(tree) => {
            print_deck_tree(&tree, 0);
        }
        Err(e) => {
            println!("Could not retrieve deck tree: {}", e);
        }
    }

    Ok(())
}

fn print_deck_info(index: usize, deck: &Deck) {
    println!("\n{}. Deck: {} (ID: {})", index, deck.name(), deck.id().0);

    // Print hierarchical information
    if deck.is_subdeck() {
        println!("   Type: Subdeck");
        if let Some(parent) = deck.parent_name() {
            println!("   Parent: {}", parent);
        }
        println!("   Base name: {}", deck.base_name());
    } else {
        println!("   Type: Root deck");
    }
}

fn print_deck_tree(nodes: &[ankiconnect_rs::client::request::DeckTreeNode], indent: usize) {
    for node in nodes {
        // Print with proper indentation to show hierarchy
        let indent_str = "  ".repeat(indent);
        println!(
            "{}{}└─ {} (ID: {})",
            indent_str,
            if indent > 0 { "" } else { "" },
            node.name,
            node.id
        );

        // Recursively print children
        if node.has_children {
            print_deck_tree(&node.children, indent + 1);
        }
    }
}
