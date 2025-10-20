use ankiconnect_rs::{AnkiClient, DuplicateScope, NoteBuilder, NoteId, QueryBuilder, Result};
use httpmock::prelude::*;
use serde_json::json;

// Helper function to create a mock AnkiClient connected to the given mock server
fn create_mock_client(server: &MockServer) -> AnkiClient {
    AnkiClient::with_connection(&server.host(), server.port())
}

#[test]
fn test_add_note() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    // Mock for getting deck info
    let deck_info_mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "deckNamesAndIds",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {"Default": 1},
                "error": null
            }));
    });

    // Mock for getting model info
    let model_info_mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelNamesAndIds",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {"Basic": 1483883011648_u64},
                "error": null
            }));
    });

    // Mock for getting model fields
    let model_fields_mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelFieldNames",
            "version": 6,
            "params": {
                "modelName": "Basic"
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": ["Front", "Back"],
                "error": null
            }));
    });

    // Mock for adding note
    let add_note_mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "addNote",
            "version": 6,
            "params": {
                "note": {
                    "deckName": "Default",
                    "modelName": "Basic",
                    "fields": {
                        "Front": "front content",
                        "Back": "back content"
                    },
                    "options": {
                        "allowDuplicate": false,
                        "duplicateScope": "deck"
                    },
                    "tags": ["test-tag"]
                }
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": 1496198395707_u64,
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Get deck and model
    let deck = client.decks().get_by_name("Default");

    deck_info_mock.assert();
    let deck = deck?.unwrap();

    let model = client.models().get_by_name("Basic");

    model_info_mock.assert();
    model_fields_mock.assert();
    let model = model?.unwrap();

    // Create note using builder
    let front_field = model.field_ref("Front").unwrap();
    let back_field = model.field_ref("Back").unwrap();

    let note = NoteBuilder::new(model.clone())
        .with_field(front_field, "front content")
        .with_field(back_field, "back content")
        .with_tag("test-tag")
        .build()
        .unwrap();

    // Act
    let note_id = client
        .cards()
        .add_note(&deck, note, false, Some(DuplicateScope::Deck));

    // Assert
    add_note_mock.assert();

    let note_id = note_id?;
    assert_eq!(note_id.value(), 1496198395707);

    Ok(())
}

#[test]
fn test_find_notes() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "findNotes",
            "version": 6,
            "params": {
                "query": "deck:current"
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": [123, 1234],
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    let query = QueryBuilder::new().in_deck("current").build();
    // Act
    let notes = client.cards().find_notes(&query);

    // Assert
    mock.assert();

    let notes = notes?;
    assert_eq!(notes.len(), 2);
    assert_eq!(notes[0].value(), 123);
    assert_eq!(notes[1].value(), 1234);

    Ok(())
}

#[test]
fn test_get_note_info() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "noteInfo",
            "version": 6,
            "params": {
                "note": 1502298033753_u64
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "noteId": 1502298033753_u64,
                    "modelName": "Basic",
                    "tags": ["tag", "another_tag"],
                    "fields": {
                        "Front": {"value": "front content", "order": 0},
                        "Back": {"value": "back content", "order": 1}
                    },
                    "cards": [1498938915662_u64]
                },
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let note_info = client.cards().get_note_info(NoteId(1502298033753));

    // Assert
    mock.assert();

    let note_info = note_info?;
    assert_eq!(note_info.note_id, 1502298033753);
    assert_eq!(note_info.model_name, "Basic");
    assert_eq!(note_info.tags, vec!["tag", "another_tag"]);
    assert_eq!(
        note_info.fields.get("Front").unwrap().value,
        "front content"
    );
    assert_eq!(note_info.fields.get("Back").unwrap().value, "back content");

    Ok(())
}

// #[test]
// fn test_update_note_fields() -> Result<()> {
//     // Arrange
//     let server = MockServer::start();
//
//     let mock = server.mock(|when, then| {
//         when.method(POST)
//             .path("/")
//             .json_body(json!({
//                 "action": "updateNoteFields",
//                 "version": 6,
//                 "params": {
//                     "note": {
//                         "id": 1514547547030_u64,
//                         "fields": {
//                             "Front": "new front content",
//                             "Back": "new back content"
//                         },
//                         "audio": []
//                     }
//                 }
//             }));
//
//         then.status(200)
//             .header("content-type", "application/json")
//             .json_body(json!({
//                 "result": null,
//                 "error": null
//             }));
//     });
//
//     let client = create_mock_client(&server);
//
//     // Act
//     let mut fields = HashMap::new();
//     fields.insert("Front".to_string(), "new front content".to_string());
//     fields.insert("Back".to_string(), "new back content".to_string());
//
//     let result = client.cards().update_note(NoteId(1514547547030), fields, None);
//
//     // Assert
//     mock.assert();
//     assert!(result.is_ok());
//
//     Ok(())
// }

// #[test]
// fn test_add_tags() -> Result<()> {
//     // Arrange
//     let server = MockServer::start();
//
//     let mock = server.mock(|when, then| {
//         when.method(POST)
//             .path("/")
//             .json_body(json!({
//                 "action": "addTags",
//                 "version": 6,
//                 "params": {
//                     "notes": [1483959289817, 1483959291695],
//                     "tags": "european-languages"
//                 }
//             }));
//
//         then.status(200)
//             .header("content-type", "application/json")
//             .json_body(json!({
//                 "result": null,
//                 "error": null
//             }));
//     });
//
//     let client = create_mock_client(&server);
//
//     // Act
//     let result = client.cards().add_tags(
//         &[NoteId(1483959289817), NoteId(1483959291695)],
//         "european-languages"
//     );
//
//     // Assert
//     mock.assert();
//     assert!(result.is_ok());
//
//     Ok(())
// }

// #[test]
// fn test_remove_tags() -> Result<()> {
//     // Arrange
//     let server = MockServer::start();
//
//     let mock = server.mock(|when, then| {
//         when.method(POST)
//             .path("/")
//             .json_body(json!({
//                 "action": "removeTags",
//                 "version": 6,
//                 "params": {
//                     "notes": [1483959289817, 1483959291695],
//                     "tags": "european-languages"
//                 }
//             }));
//
//         then.status(200)
//             .header("content-type", "application/json")
//             .json_body(json!({
//                 "result": null,
//                 "error": null
//             }));
//     });
//
//     let client = create_mock_client(&server);
//
//     // Act
//     let result = client.cards().remove_tags(
//         &[NoteId(1483959289817), NoteId(1483959291695)],
//         "european-languages"
//     );
//
//     // Assert
//     mock.assert();
//     assert!(result.is_ok());
//
//     Ok(())
// }

// #[test]
// fn test_get_tags() -> Result<()> {
//     // Arrange
//     let server = MockServer::start();
//
//     let mock = server.mock(|when, then| {
//         when.method(POST)
//             .path("/")
//             .json_body(json!({
//                 "action": "getTags",
//                 "version": 6
//             }));
//
//         then.status(200)
//             .header("content-type", "application/json")
//             .json_body(json!({
//                 "result": ["european-languages", "idioms"],
//                 "error": null
//             }));
//     });
//
//     let client = create_mock_client(&server);
//
//     // Act
//     let tags = client.cards().get_tags();
//
//     // Assert
//     mock.assert();
//
//     let tags = tags?;
//     assert_eq!(tags.len(), 2);
//     assert!(tags.contains(&"european-languages".to_string()));
//     assert!(tags.contains(&"idioms".to_string()));
//
//     Ok(())
// }

#[test]
fn test_delete_notes() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "deleteNotes",
            "version": 6,
            "params": {
                "notes": [1502298033753_u64]
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
    let result = client.cards().delete_notes(&[NoteId(1502298033753)]);

    // Assert
    mock.assert();
    assert!(result.is_ok());

    Ok(())
}
