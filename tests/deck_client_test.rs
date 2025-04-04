use ankiconnect_rs::{AnkiClient, AnkiConnectError, AnkiError, DeckId, Result};
use httpmock::prelude::*;
use serde_json::json;

// Helper function to create a mock AnkiClient connected to the given mock server
fn create_mock_client(server: &MockServer) -> AnkiClient {
    AnkiClient::with_connection(&server.host(), server.port())
}

#[test]
fn test_get_all_decks() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "deckNamesAndIds",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "Default": 1,
                    "Japanese::Vocabulary": 1494723142483_u64,
                    "Spanish::Grammar": 1494703460437_u64
                },
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let decks = client.decks().get_all();

    // Assert
    mock.assert();

    let decks = decks?;
    assert_eq!(decks.len(), 3);

    // Verify the returned decks contain expected values
    let default_deck = decks.iter().find(|d| d.name() == "Default").unwrap();
    assert_eq!(default_deck.id().0, 1);

    let japanese_deck = decks
        .iter()
        .find(|d| d.name() == "Japanese::Vocabulary")
        .unwrap();
    assert_eq!(japanese_deck.id().0, 1494723142483);

    let spanish_deck = decks
        .iter()
        .find(|d| d.name() == "Spanish::Grammar")
        .unwrap();
    assert_eq!(spanish_deck.id().0, 1494703460437);

    Ok(())
}

#[test]
fn test_get_deck_by_name() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "deckNamesAndIds",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "Default": 1,
                    "Japanese::Vocabulary": 1494723142483_u64
                },
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let deck = client.decks().get_by_name("Japanese::Vocabulary");

    // Assert
    mock.assert();
    let deck = deck?;
    assert!(deck.is_some());
    let deck = deck.unwrap();
    assert_eq!(deck.name(), "Japanese::Vocabulary");
    assert_eq!(deck.id().0, 1494723142483);

    // Test non-existent deck
    let non_existent = client.decks().get_by_name("NonExistent")?;
    assert!(non_existent.is_none());

    Ok(())
}

#[test]
fn test_get_deck_by_id() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "deckNamesAndIds",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "Default": 1,
                    "Japanese::Vocabulary": 1494723142483_u64
                },
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let deck = client.decks().get_by_id(DeckId(1494723142483));

    // Assert
    mock.assert();
    let deck = deck?;
    assert!(deck.is_some());
    let deck = deck.unwrap();
    assert_eq!(deck.name(), "Japanese::Vocabulary");
    assert_eq!(deck.id().0, 1494723142483);

    // Test non-existent deck ID
    let non_existent = client.decks().get_by_id(DeckId(99999))?;
    assert!(non_existent.is_none());

    Ok(())
}

#[test]
fn test_create_deck() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "createDeck",
            "version": 6,
            "params": {
                "deck": "German::Vocabulary"
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": 1519323742721_u64,
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let deck_id = client.decks().create("German::Vocabulary")?;

    // Assert
    mock.assert();
    assert_eq!(deck_id.0, 1519323742721);

    Ok(())
}

#[test]
fn test_create_empty_deck_name() {
    // Arrange
    let server = MockServer::start();
    let client = create_mock_client(&server);

    // Act
    let result = client.decks().create("");

    // Assert
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("cannot be empty"));
    }
}

#[test]
fn test_delete_deck() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    // Mock for delete request
    let delete_mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "deleteDecks",
            "version": 6,
            "params": {
                "decks": ["Japanese::JLPT N5"],
                "cardsToo": true
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": null,
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let result = client.decks().delete("Japanese::JLPT N5", true);

    // Assert
    delete_mock.assert();
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_get_configurations() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "getDeckConfig",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "current_deck_id": 1,
                    "current_config_id": 1,
                    "all_config_id": [1, 2],
                    "config_list": [
                        {
                            "id": 1,
                            "name": "Default",
                            "reuse_if_possible": true,
                            "disable_auto_qe": false
                        },
                        {
                            "id": 2,
                            "name": "Custom",
                            "reuse_if_possible": false,
                            "disable_auto_qe": true
                        }
                    ]
                },
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let configs = client.decks().get_configurations()?;

    // Assert
    mock.assert();
    assert_eq!(configs.len(), 2);

    let default_config = configs.iter().find(|c| c.name == "Default").unwrap();
    assert_eq!(default_config.id, 1);
    assert!(default_config.reuse_if_possible);
    assert!(!default_config.disable_auto_qe);

    let custom_config = configs.iter().find(|c| c.name == "Custom").unwrap();
    assert_eq!(custom_config.id, 2);
    assert!(!custom_config.reuse_if_possible);
    assert!(custom_config.disable_auto_qe);

    Ok(())
}

#[test]
fn test_get_tree() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "deckTree",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": [
                    {
                        "id": 1,
                        "name": "Default",
                        "level": 0,
                        "collapsed": false,
                        "has_children": false,
                        "children": []
                    },
                    {
                        "id": 1234567890,
                        "name": "Japanese",
                        "level": 0,
                        "collapsed": false,
                        "has_children": true,
                        "children": [
                            {
                                "id": 1494723142483_u64,
                                "name": "Japanese::Vocabulary",
                                "level": 1,
                                "collapsed": false,
                                "has_children": false,
                                "children": []
                            }
                        ]
                    }
                ],
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let tree = client.decks().get_tree()?;

    // Assert
    mock.assert();
    assert_eq!(tree.len(), 2);

    // Check root nodes
    let default_node = tree.iter().find(|node| node.name == "Default").unwrap();
    assert_eq!(default_node.id, 1);
    assert_eq!(default_node.level, 0);
    assert!(!default_node.has_children);
    assert!(default_node.children.is_empty());

    let japanese_node = tree.iter().find(|node| node.name == "Japanese").unwrap();
    assert_eq!(japanese_node.id, 1234567890);
    assert_eq!(japanese_node.level, 0);
    assert!(japanese_node.has_children);
    assert_eq!(japanese_node.children.len(), 1);

    // Check child node
    let vocab_node = &japanese_node.children[0];
    assert_eq!(vocab_node.name, "Japanese::Vocabulary");
    assert_eq!(vocab_node.id, 1494723142483);
    assert_eq!(vocab_node.level, 1);
    assert!(!vocab_node.has_children);
    assert!(vocab_node.children.is_empty());

    Ok(())
}

#[test]
fn test_get_stats() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    // Mock for stats request
    let stats_mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "getDeckStats",
            "version": 6,
            "params": {
                "decks": ["Japanese::Reading"]
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "1736956963663": {
                        "deck_id": 1736956963663_u64,
                        "name": "Reading",
                        "new_count": 500,
                        "learn_count": 3,
                        "review_count": 127,
                        "total_in_deck": 13788
                    }},
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let stats = client.decks().get_stat("Japanese::Reading");

    // Assert
    stats_mock.assert();
    let stats = stats?;
    assert_eq!(stats.deck_id, 1736956963663_u64);
    assert_eq!(stats.new_count, 500);
    assert_eq!(stats.learn_count, 3);
    assert_eq!(stats.review_count, 127);
    assert_eq!(stats.total_in_deck, 13788);

    Ok(())
}

#[test]
fn test_get_cards_in_deck() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    // Mock for findCards request
    let cards_mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "findCards",
            "version": 6,
            "params": {
                "query":"deck:Japanese\\:\\:Vocabulary"
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": [1111111, 2222222, 3333333],
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let cards = client.decks().get_cards_in_deck("Japanese::Vocabulary");

    // Assert
    cards_mock.assert();
    let cards = cards?;
    assert_eq!(cards.len(), 3);
    assert_eq!(cards[0].0, 1111111);
    assert_eq!(cards[1].0, 2222222);
    assert_eq!(cards[2].0, 3333333);

    Ok(())
}

#[test]
fn test_exists() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "deckNamesAndIds",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "Default": 1,
                    "Japanese::Vocabulary": 1494723142483_u64
                },
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act & Assert
    assert!(client.decks().exists("Default")?);
    assert!(client.decks().exists("Japanese::Vocabulary")?);
    assert!(!client.decks().exists("NonExistent")?);

    Ok(())
}

#[test]
fn test_deck_not_found_error() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    // Mock for getDeckStats request with error
    let deck_stats_mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "getDeckStats",
            "version": 6,
            "params": {
                "decks": ["Default"]
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": null,
                "error": "deck was not found: Default"
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let result = client.decks().get_stat("Default");

    // Assert
    deck_stats_mock.assert();
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        AnkiError::AnkiConnectError(AnkiConnectError::DeckNotFound(_))
    ));
    Ok(())
}

#[test]
fn test_server_not_running() {
    // Arrange
    // Start and immediately stop a mock server to get a known-unused port
    let server = MockServer::start();
    let port = server.port();
    let host = server.host();
    drop(server); // Server stops here, port becomes unavailable

    let client = AnkiClient::with_connection(&host, port);

    // Act
    let result = client.decks().get_all();

    // Assert
    assert!(result.is_err());
    let error_string = result.unwrap_err().to_string();
    assert!(
        error_string.contains("connection refused")
            || error_string.contains("failed to connect")
            || error_string.contains("404")
    );
}
