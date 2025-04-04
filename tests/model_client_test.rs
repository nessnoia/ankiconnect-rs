use ankiconnect_rs::{AnkiClient, Result};
use httpmock::prelude::*;
use serde_json::json;

// Helper function to create a mock AnkiClient connected to the given mock server
fn create_mock_client(server: &MockServer) -> AnkiClient {
    AnkiClient::with_connection(&server.host(), server.port())
}

#[test]
fn test_get_all_models() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let model_names_mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelNamesAndIds",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "Basic": 1483883011648_u64,
                    "Basic (and reversed card)": 1483883011644_u64,
                    "Basic (optional reversed card)": 1483883011631_u64,
                    "Cloze": 1483883011630_u64
                },
                "error": null
            }));
    });

    // Also mock the field names request for each model
    let model_fields_mock1 = server.mock(|when, then| {
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

    let model_fields_mock2 = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelFieldNames",
            "version": 6,
            "params": {
                "modelName": "Basic (and reversed card)"
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": ["Front", "Back"],
                "error": null
            }));
    });

    let model_fields_mock3 = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelFieldNames",
            "version": 6,
            "params": {
                "modelName": "Basic (optional reversed card)"
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": ["Front", "Back", "Add Reverse"],
                "error": null
            }));
    });

    let model_fields_mock4 = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelFieldNames",
            "version": 6,
            "params": {
                "modelName": "Cloze"
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": ["Text", "Extra"],
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let models = client.models().get_all();

    // Assert
    model_names_mock.assert();
    model_fields_mock1.assert();
    model_fields_mock2.assert();
    model_fields_mock3.assert();
    model_fields_mock4.assert();

    let models = models?;
    assert_eq!(models.len(), 4);

    // Verify the models
    let basic = models.iter().find(|m| m.name() == "Basic").unwrap();
    assert_eq!(basic.id().0, 1483883011648);
    assert_eq!(basic.fields().len(), 2);
    assert_eq!(basic.fields()[0].name(), "Front");
    assert_eq!(basic.fields()[1].name(), "Back");

    let cloze = models.iter().find(|m| m.name() == "Cloze").unwrap();
    assert_eq!(cloze.id().0, 1483883011630);
    assert_eq!(cloze.fields().len(), 2);
    assert_eq!(cloze.fields()[0].name(), "Text");
    assert_eq!(cloze.fields()[1].name(), "Extra");

    Ok(())
}

#[test]
fn test_get_model_by_name() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    // Mock for getting all models
    let model_names_mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelNamesAndIds",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "Basic": 1483883011648_u64,
                    "Cloze": 1483883011630_u64
                },
                "error": null
            }));
    });

    // Mock for getting Basic model fields
    let basic_names_mock = server.mock(|when, then| {
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

    // Mock for getting Cloze model fields
    let cloze_names_mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelFieldNames",
            "version": 6,
            "params": {
                "modelName": "Cloze"
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": ["Text", "Extra"],
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let model = client.models().get_by_name("Basic")?;

    // Assert
    model_names_mock.assert();
    basic_names_mock.assert();
    cloze_names_mock.assert();
    assert!(model.is_some());
    let model = model.unwrap();
    assert_eq!(model.name(), "Basic");
    assert_eq!(model.id().0, 1483883011648);
    assert_eq!(model.fields().len(), 2);
    assert_eq!(model.fields()[0].name(), "Front");
    assert_eq!(model.fields()[1].name(), "Back");

    // Test with non-existent model
    let non_existent = client.models().get_by_name("NonExistent")?;
    assert!(non_existent.is_none());

    Ok(())
}

#[test]
fn test_get_model_field_names() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
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

    let client = create_mock_client(&server);

    // Act
    let field_names = client.models().get_fields_for_name("Basic");

    // Assert
    mock.assert();

    let field_names = field_names?;
    assert_eq!(field_names.len(), 2);
    assert_eq!(field_names[0], "Front");
    assert_eq!(field_names[1], "Back");

    Ok(())
}

#[ignore]
#[test]
fn test_get_model_templates() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    // First, mock the modelNamesAndIds request to get model IDs
    server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelNamesAndIds",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "Basic (and reversed card)": 1483883011644_u64
                },
                "error": null
            }));
    });

    // Mock for getting the model fields
    server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelFieldNames",
            "version": 6,
            "params": {
                "modelName": "Basic (and reversed card)"
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": ["Front", "Back"],
                "error": null
            }));
    });

    // Mock the templates request
    let templates_mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelTemplates",
            "version": 6,
            "params": {
                "modelName": "Basic (and reversed card)"
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "Card 1": {
                        "Front": "{{Front}}",
                        "Back": "{{FrontSide}}\n\n<hr id=answer>\n\n{{Back}}"
                    },
                    "Card 2": {
                        "Front": "{{Back}}",
                        "Back": "{{FrontSide}}\n\n<hr id=answer>\n\n{{Front}}"
                    }
                },
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // First, get the model
    let model = client.models().get_by_name("Basic (and reversed card)")?;
    assert!(model.is_some());
    let model = model.unwrap();

    // Act
    let templates = client.models().get_template_names(&model);

    // Assert
    templates_mock.assert();

    let templates = templates?;
    assert_eq!(templates.len(), 2);
    assert!(templates.contains(&"Card 1".to_string()));
    assert!(templates.contains(&"Card 2".to_string()));

    Ok(())
}

#[ignore]
#[test]
fn test_get_model_styling() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    // First, mock the modelNamesAndIds request to get model IDs
    server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelNamesAndIds",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "Basic (and reversed card)": 1483883011644_u64
                },
                "error": null
            }));
    });

    // Mock for getting the model fields
    server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelFieldNames",
            "version": 6,
            "params": {
                "modelName": "Basic (and reversed card)"
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": ["Front", "Back"],
                "error": null
            }));
    });

    // Mock the styling request
    let styling_mock = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .json_body(json!({
                "action": "modelStyling",
                "version": 6,
                "params": {
                    "modelName": "Basic (and reversed card)"
                }
            }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "css": ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n"
                },
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // First, get the model
    let model = client.models().get_by_name("Basic (and reversed card)")?;
    assert!(model.is_some());
    let model = model.unwrap();

    // Act
    let styling = client.models().get_styling(&model);

    // Assert
    styling_mock.assert();

    let styling = styling?;
    assert!(styling.contains("font-family: arial"));
    assert!(styling.contains("font-size: 20px"));
    assert!(styling.contains("text-align: center"));

    Ok(())
}

#[ignore]
#[test]
fn test_create_model() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "createModel",
            "version": 6,
            "params": {
                "modelName": "TestModel",
                "inOrderFields": ["Field1", "Field2"],
                "css": ".card { font-family: arial; }",
                "cardTemplates": {
                    "Card 1": {
                        "Front": "Front template {{Field1}}",
                        "Back": "Back template {{Field2}}"
                    }
                }
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": 1551462107104_u64,
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let model_id = client.models().create_model(
        "TestModel",
        &["Field1", "Field2"],
        ".card { font-family: arial; }",
        &[(
            "Card 1",
            "Front template {{Field1}}",
            "Back template {{Field2}}",
        )],
    );

    // Assert
    mock.assert();

    let model_id = model_id?;
    assert_eq!(model_id.0, 1551462107104);

    Ok(())
}

#[ignore]
#[test]
fn test_update_model_styling() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    // First, mock the modelNamesAndIds request to get model IDs
    server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "modelNamesAndIds",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": {
                    "Basic": 1483883011648_u64
                },
                "error": null
            }));
    });

    // Mock for getting the model fields
    server.mock(|when, then| {
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

    // Mock the update styling request
    let update_mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "updateModelStyling",
            "version": 6,
            "params": {
                "model": {
                    "name": "Basic",
                    "css": ".card { font-family: verdana; }"
                }
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

    // First, get the model
    let model = client.models().get_by_name("Basic")?;
    assert!(model.is_some());
    let model = model.unwrap();

    // Act
    let result = client
        .models()
        .update_styling(&model, ".card { font-family: verdana; }");

    // Assert
    update_mock.assert();
    assert!(result.is_ok());

    Ok(())
}
