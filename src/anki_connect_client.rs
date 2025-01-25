use crate::anki_card::{AnkiCard, AnkiDeck, AnkiModel, AnkiModelIdentifier};
use crate::error::{parse_anki_connect_error, AnkiConnectError, AnkiRequestError};
use crate::request_sender::{AnkiConnectRequestSender, HttpAnkiConnectRequestSender};
use crate::types::{CardsReordering, DuplicateScope};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

const ANKI_CONNECT_API_VERSION: u8 = 6;

#[derive(Serialize)]
pub struct AnkiConnectRequest<T> {
    action: &'static str,
    version: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<T>,
}

#[derive(Deserialize)]
pub struct AnkiConnectResponse<T> {
    result: Option<T>,
    error: Option<String>,
}

impl<T> AnkiConnectResponse<T> {
    pub fn into_result(self) -> Result<T, AnkiRequestError> {
        if let Some(error) = self.error {
            let anki_error = parse_anki_connect_error(&error);
            Err(anki_error.into())
        } else if let Some(result) = self.result {
            Ok(result)
        } else {
            Err(AnkiConnectError::Other(
                "Both result and error are null. Probably a bug in AnkiConnect".to_string(),
            )
            .into())
        }
    }
}

pub struct AnkiClient<T: AnkiConnectRequestSender> {
    request_sender: T,
}

impl Default for AnkiClient<HttpAnkiConnectRequestSender> {
    fn default() -> Self {
        Self {
            request_sender: HttpAnkiConnectRequestSender::new("localhost", 8765),
        }
    }
}

impl<T: AnkiConnectRequestSender> AnkiClient<T> {
    pub fn new(url: &str, port: u16) -> AnkiClient<HttpAnkiConnectRequestSender> {
        AnkiClient {
            request_sender: HttpAnkiConnectRequestSender::new(url, port),
        }
    }

    /// Returns an array of card IDs for a given query.
    pub fn find_cards(&self, query: &str) -> Result<Vec<u64>, AnkiRequestError> {
        #[derive(Serialize)]
        struct FindCardsParams<'a> {
            query: &'a str,
        }

        let request = AnkiConnectRequest {
            action: "findCards",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(FindCardsParams { query }),
        };

        self.request_sender.send_request(request)?.into_result()
    }

    /// Gets the complete list of Anki deck names for the current user.
    pub fn get_all_decks(&self) -> Result<Vec<AnkiDeck>, AnkiRequestError> {
        let request = AnkiConnectRequest::<Option<()>> {
            action: "deckNamesAndIds",
            version: ANKI_CONNECT_API_VERSION,
            params: None,
        };

        let result: Result<HashMap<String, u64>, AnkiRequestError> =
            self.request_sender.send_request(request)?.into_result();
        result.map(|hash_map| {
            hash_map
                .into_iter()
                .map(|(name, id)| AnkiDeck::new(id, name))
                .collect()
        })
    }

    /// Create a new empty deck with name 'deck_name'.
    /// Will not overwrite a deck that exists with the same name.
    ///
    /// Returns the id of the created deck
    pub fn create_deck(&self, deck_name: &str) -> Result<u64, AnkiRequestError> {
        // TODO: Might want to add an error for empty deck name
        #[derive(Serialize)]
        struct CreateDeckParams<'a> {
            deck: &'a str,
        }

        let request = AnkiConnectRequest {
            action: "createDeck",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(CreateDeckParams { deck: deck_name }),
        };

        self.request_sender.send_request(request)?.into_result()
    }

    /// Invokes the Card Browser dialog and searches for a given query.
    /// Returns an array of identifiers of the cards that were found.
    /// Query syntax is documented [here](https://docs.ankiweb.net/searching.html).
    ///
    /// Optionally, the `reorder_cards` property can be provided to reorder the cards shown in the Card Browser.
    /// This is an array including the order and columnId objects.
    /// `order` can be either ascending or descending while columnId can be one of several
    /// column identifiers (as documented in the Anki source code).
    /// The specified column needs to be visible in the Card Browser.
    pub fn gui_browse(
        &self,
        query: &str,
        reorder_cards: Option<CardsReordering>,
    ) -> Result<Vec<u64>, AnkiRequestError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct GuiBrowseParams<'a> {
            query: &'a str,
            reorder_cards: Option<CardsReordering>,
        }

        let request = AnkiConnectRequest {
            action: "guiBrowse",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(GuiBrowseParams {
                query,
                reorder_cards,
            }),
        };

        self.request_sender.send_request(request)?.into_result()
    }

    /// Stores the file at `path` inside Anki's media folder.
    /// To prevent Anki from removing files not used by any cards (e.g. for configuration files), prefix the filename with an underscore.
    /// These files are still synchronized to AnkiWeb.
    /// Set `overwrite` to false in order to prevent Anki from overwriting files with the same name.
    ///
    /// Note: The file referenced by `path` needs to be on the same device as Anki.
    ///
    /// Returns the filename that was used by Anki. Can be different from `filename` if `overwrite` is set to false.
    pub fn store_media_file_from_path<P: AsRef<Path>>(
        &self,
        path: P,
        filename: &str,
        overwrite: bool,
    ) -> Result<String, AnkiRequestError> {
        let path_str = path.as_ref().to_string_lossy();

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct StoreMediaFileFromPathParams<'a> {
            filename: &'a str,
            path: &'a str,
            delete_existing: bool,
        }

        let request = AnkiConnectRequest {
            action: "storeMediaFile",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(StoreMediaFileFromPathParams {
                filename,
                path: &path_str,
                delete_existing: overwrite,
            }),
        };

        self.request_sender.send_request(request)?.into_result()
    }

    pub fn store_media_file_from_url(
        &self,
        url: &str,
        filename: &str,
        overwrite: bool,
    ) -> Result<String, AnkiRequestError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct StoreMediaFileFromUrlParams<'a> {
            filename: &'a str,
            url: &'a str,
            delete_existing: bool,
        }

        let request = AnkiConnectRequest {
            action: "storeMediaFile",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(StoreMediaFileFromUrlParams {
                filename,
                url,
                delete_existing: overwrite,
            }),
        };

        self.request_sender.send_request(request)?.into_result()
    }

    pub fn store_media_file_from_base64(
        &self,
        data: &str,
        filename: &str,
        overwrite: bool,
    ) -> Result<String, AnkiRequestError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct StoreMediaFileFromBase64Params<'a> {
            filename: &'a str,
            data: &'a str,
            delete_existing: bool,
        }

        let request = AnkiConnectRequest {
            action: "storeMediaFile",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(StoreMediaFileFromBase64Params {
                filename,
                data,
                delete_existing: overwrite,
            }),
        };

        self.request_sender.send_request(request)?.into_result()
    }

    /// Gets the version of the AnkiConnect plugin
    pub fn get_version(&self) -> Result<u16, AnkiRequestError> {
        let request = AnkiConnectRequest::<Option<()>> {
            action: "version",
            version: ANKI_CONNECT_API_VERSION,
            params: None,
        };

        self.request_sender.send_request(request)?.into_result()
    }

    /// Returns the list of field names for the given model
    pub fn get_field_names_for_model(
        &self,
        model: &AnkiModelIdentifier,
    ) -> Result<Vec<String>, AnkiRequestError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ModelFieldNamesParams<'a> {
            model_name: &'a str,
        }

        let request = AnkiConnectRequest {
            action: "modelFieldNames",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(ModelFieldNamesParams {
                model_name: model.get_name(),
            }),
        };

        self.request_sender.send_request(request)?.into_result()
    }

    /// Gets the complete list of available Anki models for the currently active profile
    pub fn get_all_models(&self) -> Result<Vec<AnkiModelIdentifier>, AnkiRequestError> {
        let request = AnkiConnectRequest::<Option<()>> {
            action: "modelNamesAndIds",
            version: ANKI_CONNECT_API_VERSION,
            params: None,
        };

        let result: Result<HashMap<String, u64>, AnkiRequestError> =
            self.request_sender.send_request(request)?.into_result();

        result.map(|hash_map| {
            hash_map
                .into_iter()
                .map(|(name, id)| AnkiModelIdentifier::new(id, name))
                .collect()
        })
    }

    /// Returns the AnkiModel for the provided model ID
    /// This allows e.g. to store the model ID and retrieve it later irrespective of its name
    /// (which might have changed)
    pub fn get_model_by_id(&self, id: u64) -> Result<Option<AnkiModel>, AnkiRequestError> {
        // TODO: We should probably reuse this information

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(dead_code)]
        struct AnkiConnectModelObject {
            pub id: u64,
            pub name: String,
            #[serde(rename = "type")]
            pub type_: u64,
            #[serde(rename = "mod")]
            pub mod_: u64,
            pub usn: i64,
            pub sortf: i64,
            pub did: Option<i64>, // TODO: i64 is probably wrong
            pub tmpls: Vec<Template>,
            pub flds: Vec<Field>,
            pub css: String,
            pub latex_pre: String,
            pub latex_post: String,
            pub latexsvg: bool,
            pub req: Vec<Requirement>,
            pub original_stock_kind: i64,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(dead_code)]
        pub struct Template {
            pub name: String,
            pub ord: i64,
            pub qfmt: String,
            pub afmt: String,
            pub bqfmt: String,
            pub bafmt: String,
            pub did: Option<i64>,
            pub bfont: String,
            pub bsize: i64,
            pub id: u64,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(dead_code)]
        pub struct Field {
            pub name: String,
            pub ord: i64,
            pub sticky: bool,
            pub rtl: bool,
            pub font: String,
            pub size: i64,
            pub description: String,
            pub plain_text: bool,
            pub collapsed: bool,
            pub exclude_from_search: bool,
            pub id: i64,
            pub tag: Option<String>,
            pub prevent_deletion: bool,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(dead_code)]
        pub struct Requirement(pub i64, pub String, pub Vec<i64>);

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct FindModelsByIdParams<'a> {
            model_ids: &'a [u64],
        }

        let ids = [id];
        let request = AnkiConnectRequest {
            action: "findModelsById",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(FindModelsByIdParams { model_ids: &ids }),
        };

        let result: Result<Vec<AnkiConnectModelObject>, AnkiRequestError> =
            self.request_sender.send_request(request)?.into_result();

        // TODO: Clone shouldn't be necessary
        result.map(|model| {
            model
                .first()
                .map(|model| AnkiModel::new(model.id, model.name.clone()))
        })
    }

    /// Verifies that the fields of the given card actually exist in the model.
    /// AnkiConnect will silently discard fields that are not present in the model
    fn validate_fields(&self, anki_card: &AnkiCard) -> Result<(), AnkiRequestError> {
        let model = anki_card.get_model_identifier();
        let model_field_names = self.get_field_names_for_model(model)?;

        // Go over all keys in the HashMap anki_card.get_fields() and check that they are contained in `model_field_names`
        for field_name in anki_card.get_fields().keys() {
            if !model_field_names.contains(field_name) {
                return Err(AnkiRequestError::InvalidField(
                    field_name.clone(),
                    model.clone(),
                ));
            }
        }

        Ok(())
    }

    /// Creates a note using the given deck and model, with the provided field values and tags.
    /// Returns the identifier of the created note created on success
    ///
    pub fn add_note(
        &self,
        deck: AnkiDeck,
        anki_card: AnkiCard,
        allow_duplicate: bool,
        duplicate_scope: Option<DuplicateScope>,
    ) -> Result<u64, AnkiRequestError> {
        // Non-existing fields fail silently with AnkiConnect, so we need to check for them manually.
        // This is non-atomic, but a field should never disappear exactly in between anyway.
        self.validate_fields(&anki_card)?;

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct AddNoteOptions {
            allow_duplicate: bool,
            duplicate_scope: Option<DuplicateScope>,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Note<'a> {
            deck_name: &'a str,
            model_name: &'a str,
            fields: &'a HashMap<String, String>,
            options: AddNoteOptions,
            tags: &'a [&'a str],
            // TODO: Add audio, video, picture?
        }

        #[derive(Serialize)]
        struct AddNoteParams<'a> {
            note: Note<'a>,
        }

        let binding = anki_card.get_tags().collect::<Vec<_>>(); // TODO: This is dumb
        let request = AnkiConnectRequest {
            action: "addNote",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(AddNoteParams {
                note: Note {
                    deck_name: deck.get_name(),
                    model_name: anki_card.get_model_identifier().get_name(),
                    fields: anki_card.get_fields(),
                    options: AddNoteOptions {
                        allow_duplicate,
                        duplicate_scope,
                    },
                    tags: binding.as_slice(),
                },
            }),
        };

        self.request_sender.send_request(request)?.into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::anki_card::{AnkiCardBuilder, AnkiModel};
    use crate::types::{ColumnIdentifier, SortOrder};
    use httpmock::{Method::POST, MockServer};
    use serde::de::IntoDeserializer;
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

        let mock = server.mock(|when, then| {
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
                        "tags": ["yomichan"]
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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

        let deck = AnkiDeck::new(1, "Default".to_string());
        let model = AnkiModelIdentifier::new(1, "Basic".to_string());
        let card = AnkiCardBuilder::new_for_model(model)
            .add_field("Front", "front content")
            .add_field("Back", "back content")
            .add_tag("yomichan")
            .build();

        // Act
        let note_id = mock_client
            .add_note(deck, card, false, Some(DuplicateScope::Deck))
            .unwrap();

        // Assert
        mock.assert();
        assert_eq!(note_id, 1496198395707);
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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

        // Act
        let result = mock_client.find_cards("deck:current");

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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

        // Act
        let result = mock_client.gui_browse(
            "deck:current",
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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

        // Act
        let result = mock_client.get_field_names_for_model(&model);

        // Assert
        mock.assert();
        assert_eq!(result.unwrap(), ["Front", "Back"]);
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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

        // Act
        let result = mock_client.get_all_models();

        // Assert
        mock.assert();
        assert_eq!(
            result.unwrap().sort_unstable(),
            [
                AnkiModelIdentifier::new(1483883011648, "Basic".to_string()),
                AnkiModelIdentifier::new(1483883011644, "Basic (and reversed card)".to_string()),
                AnkiModelIdentifier::new(
                    1483883011631,
                    "Basic (optional reversed card)".to_string()
                ),
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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

        // Act
        let result = mock_client.store_media_file_from_path("/path/to/file", "_hello.txt", true);

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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

        // Act
        let result =
            mock_client.store_media_file_from_url("https://url.to.file", "_hello.txt", false);

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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

        // Act
        let result =
            mock_client.store_media_file_from_base64("SGVsbG8sIHdvcmxkIQ==", "_hello.txt", false);

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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

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

        let client = AnkiClient::<HttpAnkiConnectRequestSender>::new(&host, port);

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
    fn into_result_returns_error_when_both_are_none() {
        let response = AnkiConnectResponse::<()> {
            result: None,
            error: None,
        };
        let result: Result<(), AnkiRequestError> = response.into_result();
        assert!(matches!(
            result,
            Err(AnkiRequestError::AnkiConnectError(AnkiConnectError::Other(
                _
            )))
        ));
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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

        // Act
        let result = mock_client.add_note(
            AnkiDeck::new(17283, "Italian".to_string()),
            AnkiCardBuilder::new_for_model(AnkiModelIdentifier::new(1, "Basic".to_string()))
                .add_field("Front", "front content")
                .add_field("Back", "back content")
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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

        // Act
        let result = mock_client.add_note(
            AnkiDeck::new(12398, "NonExistingDeck".to_string()),
            AnkiCardBuilder::new_for_model(AnkiModelIdentifier::new(1, "Basic".to_string()))
                .add_field("Front", "front content")
                .add_field("Back", "back content")
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
        let model_fields_mock = server.mock(|when, then| {
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

        let mock_client =
            AnkiClient::<HttpAnkiConnectRequestSender>::new(&server.host(), server.port());

        let deck = AnkiDeck::new(1, "Default".to_string());
        let card = AnkiCardBuilder::new_for_model(model.clone())
            .add_field("Front", "front content")
            .add_field("Back", "back content")
            .add_field("SomeNonExistingField", "random content123")
            .build();

        // Act
        let result = mock_client.add_note(deck, card, false, None);

        // Assert
        model_fields_mock.assert();
        match result {
            Err(AnkiRequestError::InvalidField(field, res_model)) => {
                assert_eq!(field, "SomeNonExistingField");
                assert_eq!(res_model, model);
            }
            _ => panic!("Expected InvalidField error, got {:?}", result),
        }
    }
}
