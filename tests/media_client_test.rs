use ankiconnect_rs::{AnkiClient, Result};
use httpmock::prelude::*;
use serde_json::json;
use std::path::PathBuf;

// Helper function to create a mock AnkiClient connected to the given mock server
fn create_mock_client(server: &MockServer) -> AnkiClient {
    AnkiClient::with_connection(&server.host(), server.port())
}

#[test]
fn test_store_media_from_base64() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "storeMediaFile",
            "version": 6,
            "params": {
                "filename": "_hello.txt",
                "data": "SGVsbG8sIHdvcmxkIQ==",
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

    let client = create_mock_client(&server);

    // Act
    let filename = client
        .media()
        .store_from_base64("SGVsbG8sIHdvcmxkIQ==", "_hello.txt", true);

    // Assert
    mock.assert();

    let filename = filename?;
    assert_eq!(filename, "_hello.txt");

    Ok(())
}

#[test]
fn test_store_media_from_url() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "storeMediaFile",
            "version": 6,
            "params": {
                "filename": "image.jpg",
                "url": "https://example.com/image.jpg",
                "deleteExisting": false
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": "image_1.jpg",
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let filename =
        client
            .media()
            .store_from_url("https://example.com/image.jpg", "image.jpg", false);

    // Assert
    mock.assert();

    let filename = filename?;
    assert_eq!(filename, "image_1.jpg");

    Ok(())
}

#[test]
fn test_retrieve_media_file() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "retrieveMediaFile",
            "version": 6,
            "params": {
                "filename": "_hello.txt"
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": "SGVsbG8sIHdvcmxkIQ==",
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let data = client.media().retrieve_file("_hello.txt");

    // Assert
    mock.assert();

    let data = data?;
    assert_eq!(data, "SGVsbG8sIHdvcmxkIQ==");

    Ok(())
}

#[test]
fn test_delete_media_file() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "deleteMediaFile",
            "version": 6,
            "params": {
                "filename": "_hello.txt"
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
    let result = client.media().delete_file("_hello.txt");

    // Assert
    mock.assert();
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_get_media_dir_path() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "getMediaDirPath",
            "version": 6
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": "/home/user/.local/share/Anki2/Main/collection.media",
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let path = client.media().get_directory();

    // Assert
    mock.assert();

    let path = path?;
    assert_eq!(
        path,
        PathBuf::from("/home/user/.local/share/Anki2/Main/collection.media")
    );

    Ok(())
}

// #[test]
// fn test_get_media_files_names() -> Result<()> {
//     // Arrange
//     let server = MockServer::start();
//
//     let mock = server.mock(|when, then| {
//         when.method(POST)
//             .path("/")
//             .json_body(json!({
//                 "action": "getMediaFilesNames",
//                 "version": 6,
//                 "params": {
//                     "pattern": "_hell*.txt"
//                 }
//             }));
//
//         then.status(200)
//             .header("content-type", "application/json")
//             .json_body(json!({
//                 "result": ["_hello.txt"],
//                 "error": null
//             }));
//     });
//
//     let client = create_mock_client(&server);
//
//     // Act
//     let filenames = client.media().get_file_names("_hell*.txt");
//
//     // Assert
//     mock.assert();
//
//     let filenames = filenames?;
//     assert_eq!(filenames.len(), 1);
//     assert_eq!(filenames[0], "_hello.txt");
//
//     Ok(())
// }

#[test]
fn test_store_media_with_invalid_params() {
    // Arrange
    let server = MockServer::start();
    let client = create_mock_client(&server);

    // Act - Empty filename
    let result = client
        .media()
        .store_from_base64("SGVsbG8sIHdvcmxkIQ==", "", true);

    // Assert
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("cannot be empty"));
    }

    // Act - Empty data
    let result = client.media().store_from_base64("", "file.txt", true);

    // Assert
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("cannot be empty"));
    }

    // Act - Empty URL
    let result = client.media().store_from_url("", "file.txt", true);

    // Assert
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("cannot be empty"));
    }
}
