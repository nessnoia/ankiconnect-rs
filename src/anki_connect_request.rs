use crate::parameter_types::{CardsReordering, DuplicateScope, MediaSource};
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Serialize)]
pub(crate) struct AnkiConnectRequest<T> {
    pub action: &'static str,
    pub version: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<T>,
}

#[derive(Serialize)]
pub(crate) struct FindCardsParams<'a> {
    pub query: &'a str,
}

#[derive(Serialize)]
pub(crate) struct CreateDeckParams<'a> {
    pub deck: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GuiBrowseParams<'a> {
    pub query: &'a str,
    pub reorder_cards: Option<CardsReordering>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModelFieldNamesParams<'a> {
    pub model_name: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FindModelsByIdParams<'a> {
    pub model_ids: &'a [u64],
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddNoteParams<'a> {
    pub note: Note<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Note<'a> {
    pub deck_name: &'a str,
    pub model_name: &'a str,
    /// field -> content mapping
    pub fields: HashMap<&'a str, &'a str>,
    pub options: AddNoteOptions<'a>,
    pub tags: Vec<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audio: Vec<Media<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub video: Vec<Media<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub picture: Vec<Media<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Media<'a> {
    #[serde(flatten)]
    pub media_source: MediaSourceDTO<'a>,

    pub filename: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_hash: Option<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<&'a str>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddNoteOptions<'a> {
    pub allow_duplicate: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_scope: Option<DuplicateScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_scope_options: Option<DuplicateScopeOptions<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DuplicateScopeOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deck_name: Option<&'a str>,
    pub check_children: bool,
    pub check_all_models: bool,
}

#[derive(Serialize)]
pub struct MediaSourceDTO<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<&'a Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<&'a str>,
}

impl<'a> MediaSourceDTO<'a> {
    pub(crate) fn from_source(media_source: &'a MediaSource) -> Self {
        match media_source {
            MediaSource::Path(path) => Self {
                path: Some(path),
                url: None,
                data: None,
            },
            MediaSource::Url(url) => Self {
                path: None,
                url: Some(url),
                data: None,
            },
            MediaSource::Base64(data) => Self {
                path: None,
                url: None,
                data: Some(data),
            },
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StoreMediaFileParams<'a> {
    #[serde(flatten)]
    pub(crate) media_source: MediaSourceDTO<'a>,
    pub(crate) filename: &'a str,
    pub(crate) delete_existing: bool,
}
