use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub enum SortOrder {
    Ascending,
    Descending,
}

// Copied from https://github.com/ankitects/anki/blob/main/rslib/src/browser_table.rs
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
#[allow(dead_code)]
pub enum ColumnIdentifier {
    #[serde(rename = "")]
    #[default]
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

#[derive(Serialize)]
pub struct CardsReordering {
    #[serde(rename = "order")]
    pub order: SortOrder,
    #[serde(rename = "columnId")]
    pub column_id: ColumnIdentifier,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DuplicateScope {
    Deck,
    Collection,
}

pub enum MediaSource {
    /// An OS path to the file
    Path(PathBuf),
    /// A URL to the file
    Url(String),
    /// The file as a base64 encoded string
    Base64(String),
}
