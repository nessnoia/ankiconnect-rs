//! Request and response types for the AnkiConnect API
//!
//! These are internal types used for serializing requests to and from AnkiConnect.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// -------------------
// Card-related params
// -------------------

/// Parameters for finding cards
#[derive(Serialize, Debug)]
pub(crate) struct FindCardsParams<'a> {
    pub query: &'a str,
}

/// Parameters for finding notes
#[derive(Serialize, Debug)]
pub(crate) struct FindNotesParams {
    pub query: String,
}

/// Parameters for browsing cards in the GUI
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GuiBrowseParams {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reorder_cards: Option<CardsReordering>,
}

/// Card reordering options for browse
#[derive(Serialize, Debug)]
pub(crate) struct CardsReordering {
    #[serde(rename = "order")]
    pub order: SortOrder,
    #[serde(rename = "columnId")]
    pub column_id: ColumnIdentifier,
}

/// Sort order for cards
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum SortOrder {
    Ascending,
    Descending,
}

/// Column identifier for sorting
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ColumnIdentifier {
    #[serde(rename = "")]
    Custom,
    Answer,
    CardMod,
    #[serde(rename = "template")]
    Cards,
    Deck,
    #[serde(rename = "cardDue")]
    Due,
    #[serde(rename = "cardEase")]
    Ease,
    #[serde(rename = "cardLapses")]
    Lapses,
    #[serde(rename = "cardIvl")]
    Interval,
    #[serde(rename = "noteCrt")]
    NoteCreation,
    NoteMod,
    #[serde(rename = "note")]
    Notetype,
    OriginalPosition,
    Question,
    #[serde(rename = "cardReps")]
    Reps,
    #[serde(rename = "noteFld")]
    SortField,
    #[serde(rename = "noteTags")]
    Tags,
    Stability,
    Difficulty,
    Retrievability,
}

/// Parameters for deleting notes
#[derive(Serialize, Debug)]
pub(crate) struct DeleteNotesParams {
    pub notes: Vec<u64>,
}

/// Parameters for operations that use card IDs
#[derive(Serialize, Debug)]
pub(crate) struct CardIdsParams {
    pub cards: Vec<u64>,
}

/// Parameters for setting a flag
#[derive(Serialize, Debug)]
pub(crate) struct SetFlagParams {
    pub cards: Vec<u64>,
    pub flag: u8,
}

/// Parameters for note info
#[derive(Serialize, Debug)]
pub(crate) struct NoteIdParam {
    pub note: u64,
}

/// Response for note info
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoteInfo {
    pub note_id: u64,
    pub model_name: String,
    pub tags: Vec<String>,
    pub fields: HashMap<String, FieldInfo>,
}

/// Field info in note info
#[derive(Deserialize, Debug)]
pub struct FieldInfo {
    pub value: String,
    pub order: u32,
}

/// Parameters for updating a note
#[derive(Serialize, Debug)]
pub(crate) struct UpdateNoteFieldsParams {
    pub id: u64,
    pub fields: HashMap<String, String>,
}

// ------------------
// Deck-related params
// ------------------

/// Parameters for creating a deck
#[derive(Serialize, Debug)]
pub(crate) struct CreateDeckParams<'a> {
    pub deck: &'a str,
}

/// Parameters for renaming a deck
#[derive(Serialize, Debug)]
pub(crate) struct RenameDeckParams<'a> {
    pub deck: &'a str,
    pub new_name: &'a str,
}

/// Parameters for deleting a deck
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeleteDeckParams<'a> {
    pub decks: &'a [&'a str],
    pub cards_too: bool,
}

/// Parameters for deck stats
#[derive(Serialize, Debug)]
pub(crate) struct DeckStatsParams<'a> {
    pub decks: &'a [&'a str],
}

/// Response for deck stats
#[derive(Deserialize, Debug)]
pub struct DeckStatsDto {
    pub deck_id: u64,
    pub new_count: u32,
    pub learn_count: u32,
    pub review_count: u32,
    pub total_in_deck: u32,
}

/// Response for deck configuration
#[derive(Deserialize, Debug)]
pub struct DeckConfigsResult {
    pub current_deck_id: u64,
    pub current_config_id: u64,
    pub all_config_id: Vec<u64>,
    pub config_list: Vec<DeckConfigDto>,
}

/// Deck configuration
#[derive(Deserialize, Debug)]
pub struct DeckConfigDto {
    pub id: u64,
    pub name: String,
    pub reuse_if_possible: bool,
    pub disable_auto_qe: bool,
}

/// Parameters for finding cards in a deck
#[derive(Serialize, Debug)]
pub(crate) struct DeckCardParams<'a> {
    pub deck: &'a str,
}

/// Deck tree node
#[derive(Deserialize, Debug)]
pub struct DeckTreeNode {
    pub id: u64,
    pub name: String,
    pub level: u32,
    pub collapsed: bool,
    pub has_children: bool,
    pub children: Vec<DeckTreeNode>,
}

// ------------------
// Media-related params
// ------------------

/// Parameters for storing media files
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StoreMediaFileParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    pub filename: String,
    pub delete_existing: bool,
}

/// Parameters for retrieving media files
#[derive(Serialize, Debug)]
pub(crate) struct RetrieveMediaParams {
    pub filename: String,
}

/// Parameters for deleting media files
#[derive(Serialize, Debug)]
pub(crate) struct DeleteMediaParams {
    pub filename: String,
}

// -------------------
// Model-related params
// -------------------

/// Parameters for getting model field names
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModelFieldNamesParams<'a> {
    pub model_name: &'a str,
}

/// Parameters for finding models by ID
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FindModelsByIdParams<'a> {
    pub model_ids: &'a [u64],
}

/// Parameters for getting model templates
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModelTemplatesParams<'a> {
    pub model_name: &'a str,
}

/// Parameters for getting model styling
#[derive(Serialize, Debug)]
pub(crate) struct ModelStylingParams<'a> {
    pub model_name: &'a str,
}

/// Parameters for updating model styling
#[derive(Serialize, Debug)]
pub(crate) struct UpdateModelStylingParams<'a> {
    pub model: &'a str,
    pub css: &'a str,
}

/// Parameters for creating a model
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateModelParams<'a> {
    pub model_name: &'a str,
    pub in_order_fields: &'a [&'a str],
    pub css: &'a str,
    pub card_templates: HashMap<String, CardTemplate>,
}

/// Card template for model creation
#[derive(Serialize, Debug)]
pub(crate) struct CardTemplate {
    pub front: String,
    pub back: String,
}

/// Model details from API response
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelDetails {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: u64,
    #[serde(rename = "mod")]
    pub mod_: u64,
    pub usn: i64,
    pub sortf: i64,
    pub did: Option<i64>,
    pub tmpls: Vec<Template>,
    pub flds: Vec<Field>,
    pub css: String,
    pub latex_pre: String,
    pub latex_post: String,
    pub latexsvg: bool,
    pub req: Vec<Requirement>,
    pub original_stock_kind: i64,
}

/// Template in model details
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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

/// Field in model details
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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

/// Requirement in model details
#[derive(Deserialize, Debug)]
pub struct Requirement(pub i64, pub String, pub Vec<i64>);

// -------------------
// Note-related params
// -------------------

/// Parameters for adding a note
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddNoteParams {
    pub note: NoteDto,
}

/// Note data for adding to Anki
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NoteDto {
    // TODO Or id?
    pub deck_name: String,
    pub model_name: String,
    /// field -> content mapping
    pub fields: HashMap<String, String>,
    pub options: AddNoteOptions,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audio: Vec<Media>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub video: Vec<Media>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub picture: Vec<Media>,
}

/// Media data for notes
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Media {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    pub filename: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<String>,
}

/// Options for adding notes
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddNoteOptions {
    pub allow_duplicate: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_scope: Option<DuplicateScopeDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_scope_options: Option<DuplicateScopeOptionsDto>,
}

/// Scope for duplicate checking
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum DuplicateScopeDto {
    Deck,
    Collection,
}

/// Options for duplicate scope
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DuplicateScopeOptionsDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deck_name: Option<String>,
    pub check_children: bool,
    pub check_all_models: bool,
}
