use ankiconnect_rs::anki_card::{
    AnkiCardBuilder, AnkiDeck, AnkiField, AnkiModel, AnkiModelIdentifier, ModelId,
};
use ankiconnect_rs::anki_search_query::AnkiSearchQueryBuilder;
use ankiconnect_rs::error::{AnkiConnectError, AnkiRequestError};
use ankiconnect_rs::parameter_types::{
    CardsReordering, ColumnIdentifier, DuplicateScope, MediaSource, SortOrder,
};
use ankiconnect_rs::AnkiClient;
use httpmock::{Method::POST, MockServer};
use serde_json::json;

#[test]
fn test_add_note_sends_correct_request() {
    // Arrange
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelFieldNames",
            "version": 6,
            "params": { "modelName": "Basic" }
        }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": ["Front", "Back"],
                "error": null
            }));
    });
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
                    "tags": ["yomichan"],
                    "audio": [{
                        "url": "https://assets.languagepod101.com/dictionary/japanese/audiomp3.php?kanji=猫&kana=ねこ",
                        "filename": "yomichan_ねこ_猫.mp3",
                        "fields": [
                            "Front"
                        ]
                    }],
                    "video": [{
                        "url": "https://cdn.videvo.net/small_watermarked/Contador_Glam_preview.mp4",
                        "filename": "countdown.mp4",
                        "fields": [
                            "Back"
                        ]
                    }],
                    "picture": [{
                        "url": "https://upload.wikimedia.org/wikipedia/220px-A_black_cat_named_Tilly.jpg",
                        "filename": "black_cat.jpg",
                        "fields": [
                            "Back"
                        ]
                    }]
                }
            }
        }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": 1496198395707u64,
                "error": null
            }));
    });
    let mock_client = AnkiClient::new(&server.host(), server.port());

    let image = MediaSource::Url(
        "https://upload.wikimedia.org/wikipedia/220px-A_black_cat_named_Tilly.jpg".to_string(),
    );
    let audio = MediaSource::Url(
        "https://assets.languagepod101.com/dictionary/japanese/audiomp3.php?kanji=猫&kana=ねこ"
            .to_string(),
    );
    let video = MediaSource::Url(
        "https://cdn.videvo.net/small_watermarked/Contador_Glam_preview.mp4".to_string(),
    );

    let deck = AnkiDeck::new(1, "Default".to_string());
    let model = AnkiModelIdentifier::new(1, "Basic".to_string());
    let card = AnkiCardBuilder::new_for_model(model.clone())
        .add_field(
            AnkiField::new(model.id(), "Front".to_string()),
            "front content",
        )
        .add_field(
            AnkiField::new(model.id(), "Back".to_string()),
            "back content",
        )
        .add_tag("yomichan")
        .add_image(
            image,
            "black_cat.jpg".to_string(),
            AnkiField::new(model.id(), "Back".to_string()),
        )
        .add_audio(
            audio,
            "yomichan_ねこ_猫.mp3".to_string(),
            AnkiField::new(model.id(), "Front".to_string()),
        )
        .add_video(
            video,
            "countdown.mp4".to_string(),
            AnkiField::new(model.id(), "Back".to_string()),
        )
        .build();

    // Act
    let note_id = mock_client.add_note(deck, card, false, Some(DuplicateScope::Deck));

    // Assert
    add_note_mock.assert();
    assert_eq!(note_id.unwrap(), 1496198395707);
}

#[test]
fn test_get_version_request() {
    // Arrange
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
        "action": "version",
        "version": 6
                    }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
            "result": 6,
            "error": null
                        }));
    });

    let mock_client = AnkiClient::new(&server.host(), server.port());

    // Act
    let result = mock_client.get_version();

    // Assert
    mock.assert();
    assert_eq!(result.unwrap(), 6);
}

#[test]
fn test_find_cards_request() {
    // Arrange
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
        "action": "findCards",
        "version": 6,
        "params": {
            "query": "deck:current"
        }
                            }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
            "result": [1494723142483u64, 1494703460437u64, 1494703479525u64],
            "error": null
                                    }));
    });

    let mock_client = AnkiClient::new(&server.host(), server.port());
    let query = AnkiSearchQueryBuilder::new().deck("current").build();

    // Act
    let result = mock_client.find_cards(&query);

    // Assert
    mock.assert();
    assert_eq!(
        result.unwrap(),
        [1494723142483u64, 1494703460437u64, 1494703479525u64]
    );
}

#[test]
fn test_get_deck_names_request() {
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
            "result": {"Default": 1},
            "error": null
                                                            }));
    });

    let mock_client = AnkiClient::new(&server.host(), server.port());

    // Act
    let result = mock_client.get_all_decks();

    // Assert
    mock.assert();
    assert_eq!(result.unwrap(), [AnkiDeck::new(1, "Default".to_string())]);
}

#[test]
fn test_create_deck_request() {
    // Arrange
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
        "action": "createDeck",
        "version": 6,
        "params": {
            "deck": "Japanese::Tokyo"
        }
                                            }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
            "result": 1519323742721u64,
            "error": null
                                                            }));
    });

    let mock_client = AnkiClient::new(&server.host(), server.port());

    // Act
    let result = mock_client.create_deck("Japanese::Tokyo");

    // Assert
    mock.assert();
    assert_eq!(result.unwrap(), 1519323742721u64);
}

#[test]
fn test_gui_browse_request() {
    // Arrange
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
        "action": "guiBrowse",
        "version": 6,
        "params": {
            "query": "deck:current",
            "reorderCards": {
                "order": "descending",
                "columnId": "noteCrt"
            }
        }
                                            }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
            "result": [1494723142483u64, 1494703460437u64, 1494703479525u64],
            "error": null
                                                            }));
    });

    let mock_client = AnkiClient::new(&server.host(), server.port());
    let query = AnkiSearchQueryBuilder::new().deck("current").build();

    // Act
    let result = mock_client.gui_browse(
        &query,
        Some(CardsReordering {
            order: SortOrder::Descending,
            column_id: ColumnIdentifier::NoteCreation,
        }),
    );

    // Assert
    mock.assert();
    assert_eq!(
        result.unwrap(),
        [1494723142483u64, 1494703460437u64, 1494703479525u64]
    );
}

#[test]
fn test_get_model_field_names_request() {
    // Arrange
    let server = MockServer::start();
    let model = AnkiModelIdentifier::new(1, "Basic".to_string());
    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
        "action": "modelFieldNames",
        "version": 6,
        "params": {
            "modelName": model.get_name()
        }
                                                            }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
            "result": ["Front", "Back"],
            "error": null
                                                                                    }));
    });

    let mock_client = AnkiClient::new(&server.host(), server.port());

    // Act
    let result = mock_client.get_fields_for_model(&model).unwrap();

    // Assert
    mock.assert();
    assert_eq!(
        result,
        [
            AnkiField::new(ModelId(1), "Front".to_string()),
            AnkiField::new(ModelId(1), "Back".to_string())
        ]
    );
}

#[test]
fn test_get_model_names_and_ids_request() {
    // Arrange
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
        "action": "modelNamesAndIds",
        "version": 6
                                                            }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
            "result": {
                "Basic": 1483883011648u64,
                "Basic (and reversed card)": 1483883011644u64,
                "Basic (optional reversed card)": 1483883011631u64,
                "Cloze": 1483883011630u64
            },
            "error": null
                                                                                    }));
    });

    let mock_client = AnkiClient::new(&server.host(), server.port());

    // Act
    let result = mock_client.get_all_models();

    // Assert
    mock.assert();
    assert_eq!(
        result.unwrap().sort_unstable(),
        [
            AnkiModelIdentifier::new(1483883011648, "Basic".to_string()),
            AnkiModelIdentifier::new(1483883011644, "Basic (and reversed card)".to_string()),
            AnkiModelIdentifier::new(1483883011631, "Basic (optional reversed card)".to_string()),
            AnkiModelIdentifier::new(1483883011630, "Cloze".to_string())
        ]
        .sort_unstable()
    );
}

#[test]
fn test_store_media_file_from_path_request() {
    // Arrange
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
        "action": "storeMediaFile",
        "version": 6,
        "params": {
            "filename": "_hello.txt",
            "path": "/path/to/file",
            "deleteExisting": true
        }
                                                            }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
            "result": "_hello.txt",
            "error": null
                                                                                    }));
    });

    let mock_client = AnkiClient::new(&server.host(), server.port());

    let media_source = MediaSource::Path("/path/to/file".into());

    // Act
    let result = mock_client.store_media_file(media_source, "_hello.txt", true);

    // Assert
    mock.assert();
    assert_eq!(result.unwrap(), "_hello.txt");
}

#[test]
fn test_store_media_file_from_url_request() {
    // Arrange
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
        "action": "storeMediaFile",
        "version": 6,
        "params": {
            "filename": "_hello.txt",
            "url": "https://url.to.file",
            "deleteExisting": false
        }
                                                                    }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
            "result": "_hello.txt",
            "error": null
                                                                                    }));
    });

    let mock_client = AnkiClient::new(&server.host(), server.port());

    let media_source = MediaSource::Url("https://url.to.file".to_string());

    // Act
    let result = mock_client.store_media_file(media_source, "_hello.txt", false);

    // Assert
    mock.assert();
    assert_eq!(result.unwrap(), "_hello.txt");
}

#[test]
fn test_store_media_file_from_base64_request() {
    // Arrange
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
        "action": "storeMediaFile",
        "version": 6,
        "params": {
            "filename": "_hello.txt",
            "data": "SGVsbG8sIHdvcmxkIQ==",
            "deleteExisting": false
        }
                                                                            }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
            "result": "_hello.txt",
            "error": null
                        }));
    });

    let mock_client = AnkiClient::new(&server.host(), server.port());

    let media_source = MediaSource::Base64("SGVsbG8sIHdvcmxkIQ==".to_string());

    // Act
    let result = mock_client.store_media_file(media_source, "_hello.txt", false);

    // Assert
    mock.assert();
    assert_eq!(result.unwrap(), "_hello.txt");
}

#[test]
fn test_get_model_by_id_request() {
    // Arrange
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
            when.method(POST).path("/").json_body(json!({
            "action": "findModelsById",
            "version": 6,
            "params": {
                "modelIds": [1704387367119u64]
            }
                                                                                        }));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
    "result": [
      {
        "id": 1704387367119u64,
        "name": "Basic",
        "type": 0,
        "mod": 1704387367u64,
        "usn": -1,
        "sortf": 0,
        "did": null,
        "tmpls": [
          {
            "name": "Card 1",
            "ord": 0,
            "qfmt": "{{Front}}",
            "afmt": "{{FrontSide}}\n\n<hr id=answer>\n\n{{Back}}",
            "bqfmt": "",
            "bafmt": "",
            "did": null,
            "bfont": "",
            "bsize": 0,
            "id": 9176047152973362695u64
          }
        ],
        "flds": [
          {
            "name": "Front",
            "ord": 0,
            "sticky": false,
            "rtl": false,
            "font": "Arial",
            "size": 20,
            "description": "",
            "plainText": false,
            "collapsed": false,
            "excludeFromSearch": false,
            "id": 2453723143453745216u64,
            "tag": null,
            "preventDeletion": false
          },
          {
            "name": "Back",
            "ord": 1,
            "sticky": false,
            "rtl": false,
            "font": "Arial",
            "size": 20,
            "description": "",
            "plainText": false,
            "collapsed": false,
            "excludeFromSearch": false,
            "id": -4853200230425436781i64,
            "tag": null,
            "preventDeletion": false
          }
        ],
        "css": ".card {\n    font-family: arial;\n    font-size: 20px;\n    text-align: center;\n    color: black;\n    background-color: white;\n}\n",
        "latexPre": "\\documentclass[12pt]{article}\n\\special{papersize=3in,5in}\n\\usepackage[utf8]{inputenc}\n\\usepackage{amssymb,amsmath}\n\\pagestyle{empty}\n\\setlength{\\parindent}{0in}\n\\begin{document}\n",
        "latexPost": "\\end{document}",
        "latexsvg": false,
        "req": [
          [
            0,
            "any",
            [
              0
            ]
          ]
        ],
        "originalStockKind": 1
      }
                    ],
    "error": null
}));
        });

    let mock_client = AnkiClient::new(&server.host(), server.port());

    // Act
    let result = mock_client.get_model_by_id(1704387367119u64);

    // Assert
    mock.assert();
    assert_eq!(
        result.unwrap(),
        Some(AnkiModel::new(1704387367119, "Basic".to_string()))
    );
}

#[test]
fn test_server_not_running() {
    // Arrange
    // Start and immediately stop a mock server to get a known-unused port
    let server = MockServer::start();
    let port = server.port();
    let host = server.host();
    drop(server); // Server stops here, port becomes unavailable

    let client = AnkiClient::new(&host, port);

    // Act
    let result = client.get_version();

    // Assert
    match result {
        Err(AnkiRequestError::HttpError(e)) => {
            let error_string = e.to_string();
            assert!(
                error_string.contains("404")
                    || error_string.contains("connection refused")
                    || error_string.contains("failed to connect")
            )
        }
        other => panic!("Expected connection error, got {:?}", other),
    }
}

#[test]
fn test_handles_model_not_found_error() {
    // Arrange
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
            "result": null,
            "error": "model was not found: Basic"
                }));
    });

    let mock_client = AnkiClient::new(&server.host(), server.port());

    // Act
    let result = mock_client.add_note(
        AnkiDeck::new(17283, "Italian".to_string()),
        AnkiCardBuilder::new_for_model(AnkiModelIdentifier::new(1, "Basic".to_string()))
            .add_field(
                AnkiField::new(ModelId(1), "Front".to_string()),
                "front content",
            )
            .add_field(
                AnkiField::new(ModelId(1), "Back".to_string()),
                "back content",
            )
            .add_tag("yomichan")
            .build(),
        false,
        Some(DuplicateScope::Deck),
    );

    // Assert
    mock.assert();
    match result {
        Err(AnkiRequestError::AnkiConnectError(AnkiConnectError::ModelNotFound(model))) => {
            assert_eq!(model, "Basic")
        }
        _ => panic!("Should error"),
    }
}

#[test]
fn test_handles_deck_not_found_error() {
    // Arrange
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
            "result": null,
            "error": "deck was not found: NonExistingDeck"
            }));
    });

    let mock_client = AnkiClient::new(&server.host(), server.port());

    // Act
    let result = mock_client.add_note(
        AnkiDeck::new(12398, "NonExistingDeck".to_string()),
        AnkiCardBuilder::new_for_model(AnkiModelIdentifier::new(1, "Basic".to_string()))
            .add_field(
                AnkiField::new(ModelId(1), "Front".to_string()),
                "front content",
            )
            .add_field(
                AnkiField::new(ModelId(1), "Back".to_string()),
                "back content",
            )
            .add_tag("yomichan")
            .build(),
        false,
        Some(DuplicateScope::Deck),
    );

    // Assert
    mock.assert();
    match result {
        Err(AnkiRequestError::AnkiConnectError(AnkiConnectError::DeckNotFound(deck))) => {
            assert_eq!(deck, "NonExistingDeck")
        }
        _ => panic!("Should error"),
    }
}

#[test]
fn test_add_note_returns_error_for_non_existing_model_field() {
    // Arrange
    let server = MockServer::start();
    let model = AnkiModelIdentifier::new(1, "Basic".to_string());
    // Mock the modelFieldNames response to return expected fields
    server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelFieldNames",
            "version": 6,
            "params": { "modelName": model.get_name() }
        }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": ["Front", "Back"], // Model only contains these fields
                "error": null
            }));
    });

    let mock_client = AnkiClient::new(&server.host(), server.port());

    let deck = AnkiDeck::new(1, "Default".to_string());
    let card = AnkiCardBuilder::new_for_model(model.clone())
        .add_field(
            AnkiField::new(ModelId(1), "Front".to_string()),
            "front content",
        )
        .add_field(
            AnkiField::new(ModelId(1), "Back".to_string()),
            "back content",
        )
        .add_field(
            AnkiField::new(ModelId(1), "SomeNonExistingField".to_string()),
            "random content123",
        )
        .build();

    // Act
    let result = mock_client.add_note(deck, card, false, None);

    // Assert
    match result {
        Err(AnkiRequestError::InvalidField(field, res_model)) => {
            assert_eq!(field, "SomeNonExistingField");
            assert_eq!(res_model, model);
        }
        _ => panic!("Expected InvalidField error, got {:?}", result),
    }
}
