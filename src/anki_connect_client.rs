use crate::anki_card::{AnkiCard, AnkiDeck, AnkiField, AnkiModel, AnkiModelIdentifier, MediaType};
use crate::anki_connect_request::{
    AddNoteOptions, AddNoteParams, AnkiConnectRequest, CreateDeckParams, FindCardsParams,
    FindModelsByIdParams, GuiBrowseParams, Media, MediaSourceDTO, ModelFieldNamesParams, Note,
};
use crate::anki_search_query::AnkiSearchQuery;
use crate::error::{parse_anki_connect_error, AnkiConnectError, AnkiRequestError};
use crate::parameter_types::{CardsReordering, DuplicateScope};
use crate::request_sender::{AnkiConnectRequestSender, HttpAnkiConnectRequestSender};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

const ANKI_CONNECT_API_VERSION: u8 = 6;

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

impl AnkiClient<HttpAnkiConnectRequestSender> {
    pub fn new(url: &str, port: u16) -> Self {
        Self {
            request_sender: HttpAnkiConnectRequestSender::new(url, port),
        }
    }
}

impl<T: AnkiConnectRequestSender> AnkiClient<T> {
    /// Returns an array of card IDs for a given query.
    pub fn find_cards(&self, query: &AnkiSearchQuery) -> Result<Vec<u64>, AnkiRequestError> {
        let request = AnkiConnectRequest {
            action: "findCards",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(FindCardsParams {
                query: query.as_str(),
            }),
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
        let request = AnkiConnectRequest {
            action: "createDeck",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(CreateDeckParams { deck: deck_name }),
        };

        self.request_sender.send_request(request)?.into_result()
    }

    /// Invokes the Card Browser dialog and searches for a given query.
    /// Returns an array of identifiers of the cards that were found.
    /// The query syntax is documented [here](https://docs.ankiweb.net/searching.html).
    ///
    /// Optionally, the `reorder_cards` property can be provided to reorder the cards shown in the Card Browser.
    /// The specified column needs to be visible in the Card Browser.
    pub fn gui_browse(
        &self,
        query: &AnkiSearchQuery,
        reorder_cards: Option<CardsReordering>,
    ) -> Result<Vec<u64>, AnkiRequestError> {
        let request = AnkiConnectRequest {
            action: "guiBrowse",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(GuiBrowseParams {
                query: query.as_str(),
                reorder_cards,
            }),
        };

        self.request_sender.send_request(request)?.into_result()
    }

    // TODO:
    // pub fn store_media_file(
    //     &self,
    //     source: MediaSource<'_>,
    //     filename: &str,
    //     overwrite: bool,
    // ) -> Result<String, AnkiRequestError> {
    //     todo!()
    // }

    // TODO: Instead of having 3 functions, create a MediaFile enum
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
    pub fn get_fields_for_model(
        &self,
        model: &AnkiModelIdentifier,
    ) -> Result<Vec<AnkiField>, AnkiRequestError> {
        let request = AnkiConnectRequest {
            action: "modelFieldNames",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(ModelFieldNamesParams {
                model_name: model.get_name(),
            }),
        };

        let fields: Result<Vec<String>, AnkiRequestError> =
            self.request_sender.send_request(request)?.into_result();
        fields.map(|names| {
            names
                .into_iter()
                .map(|name| AnkiField::new(model.id(), name))
                .collect()
        })
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

    /// Verifies that the fields of the given card actually exist in the Anki model.
    /// AnkiConnect will silently discard fields that are not present in the model
    fn validate_fields(&self, anki_card: &AnkiCard) -> Result<(), AnkiRequestError> {
        let model = anki_card.get_model_identifier();
        let model_field_names = self.get_fields_for_model(model)?;

        // Go over all keys in the HashMap anki_card.get_fields() and check that they are contained in `model_field_names`
        for field in anki_card.get_populated_fields() {
            if !model_field_names.contains(field) {
                return Err(AnkiRequestError::InvalidField(
                    field.name().to_string(),
                    model.clone(),
                ));
            }
        }

        Ok(())
    }

    fn add_note_no_validation(
        &self,
        deck: AnkiDeck,
        anki_card: AnkiCard,
        allow_duplicate: bool,
        duplicate_scope: Option<DuplicateScope>,
    ) -> Result<u64, AnkiRequestError> {
        let fields = anki_card
            .get_fields_with_content()
            .map(|(field, content)| (field.name(), &**content))
            .collect();

        let mut images = vec![];
        let mut audio = vec![];
        let mut videos = vec![];

        anki_card
            .get_media()
            .for_each(|(media_type, source, filename, field)| {
                let media = Media {
                    media_source: MediaSourceDTO::from_source(source),
                    filename,
                    skip_hash: None,
                    fields: vec![field.name()],
                };
                match media_type {
                    MediaType::Picture => images.push(media),
                    MediaType::Audio => audio.push(media),
                    MediaType::Video => videos.push(media),
                }
            });

        let request = AnkiConnectRequest {
            action: "addNote",
            version: ANKI_CONNECT_API_VERSION,
            params: Some(AddNoteParams {
                note: Note {
                    deck_name: deck.get_name(),
                    model_name: anki_card.get_model_identifier().get_name(),
                    fields,
                    options: AddNoteOptions {
                        allow_duplicate,
                        duplicate_scope,
                        duplicate_scope_options: None,
                    },
                    tags: anki_card.get_tags().collect(),
                    // TODO: Implement:
                    audio,
                    video: videos,
                    picture: images,
                },
            }),
        };

        self.request_sender.send_request(request)?.into_result()
    }

    /// Creates a note in Anki.
    ///
    /// # Arguments
    /// * `deck` - Target deck for the note.
    /// * `anki_card` - Card content and metadata.
    /// * `allow_duplicate` - Whether to allow duplicates per `duplicate_scope`.
    /// * `duplicate_scope` - Scope for duplicate checking (deck/collection).
    ///
    /// # Errors
    /// Returns `AnkiRequestError` if the card contains fields that are not present in
    /// the note type or AnkiConnect fails.
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
        self.add_note_no_validation(deck, anki_card, allow_duplicate, duplicate_scope)
    }
}

#[cfg(test)]
mod tests {
    use crate::anki_connect_client::AnkiConnectResponse;
    use crate::error::{AnkiConnectError, AnkiRequestError};

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
}
