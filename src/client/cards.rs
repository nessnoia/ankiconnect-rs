//! Client for Anki card and note operations

use std::collections::HashMap;
use std::sync::Arc;

use crate::builders::{Flag, Query};
use crate::error::Result;
use crate::http::{HttpRequestSender, RequestSender};
use crate::models::{CardId, Deck, Note, NoteId};

use super::request::{
    self, AddNoteOptions, AddNoteParams, CardsReordering, DuplicateScopeDto, FindCardsParams,
    GuiBrowseParams, Media, NoteDto,
};

/// Client for card-related operations
pub struct CardClient {
    sender: Arc<HttpRequestSender>,
}

impl CardClient {
    /// Creates a new CardClient with the given request sender
    pub(crate) fn new(sender: Arc<HttpRequestSender>) -> Self {
        Self { sender }
    }

    /// Gets the version of the AnkiConnect plugin
    pub(crate) fn get_version(&self) -> Result<u16> {
        self.sender.send::<(), u16>("version", None)
    }

    /// Adds a new note to Anki.
    ///
    /// Note that it doesn't check validity of the fields contained in `note` and will fail
    /// silently if `note` contains fields that are not existent in Anki.
    ///
    /// # Arguments
    ///
    /// * `deck` - The deck where the note will be added
    /// * `note` - The note to add
    /// * `allow_duplicate` - Whether to allow duplicate notes
    /// * `duplicate_scope` - Optional scope for duplicate checking
    ///
    /// # Returns
    ///
    /// The ID of the created note
    pub fn add_note(
        &self,
        deck: &Deck,
        note: Note,
        allow_duplicate: bool,
        duplicate_scope: Option<DuplicateScope>,
    ) -> Result<NoteId> {
        // TODO: Probably add a validity check for missing fields
        // Convert the domain note to the API format
        let note_dto = self.prepare_note_dto(deck, &note, allow_duplicate, duplicate_scope);

        // Send the request to add the note
        let params = AddNoteParams { note: note_dto };
        let note_id = self.sender.send("addNote", Some(params))?;

        Ok(NoteId(note_id))
    }

    /// Finds cards matching the given query
    ///
    /// # Arguments
    ///
    /// * `query` - The search query for cards
    ///
    /// # Returns
    ///
    /// A list of card IDs matching the query
    pub fn find(&self, query: &Query) -> Result<Vec<CardId>> {
        let params = FindCardsParams {
            query: query.as_str(),
        };
        let ids = self.sender.send::<_, Vec<u64>>("findCards", Some(params))?;
        Ok(ids.into_iter().map(CardId).collect())
    }

    /// Opens the Anki card browser with the given query
    ///
    /// # Arguments
    ///
    /// * `query` - The search query for cards
    ///
    /// # Returns
    ///
    /// A list of card IDs that were found
    pub fn browse(&self, query: &str) -> Result<Vec<CardId>> {
        let params = GuiBrowseParams {
            query: query.to_string(),
            reorder_cards: None,
        };
        let ids = self.sender.send::<_, Vec<u64>>("guiBrowse", Some(params))?;
        Ok(ids.into_iter().map(CardId).collect())
    }

    /// Opens the Anki card browser with the given query and sorts the results
    ///
    /// # Arguments
    ///
    /// * `query` - The search query for cards
    /// * `column` - The column to sort by
    /// * `ascending` - Whether to sort in ascending order
    ///
    /// # Returns
    ///
    /// A list of card IDs that were found
    pub fn browse_sorted(
        &self,
        query: &str,
        column: SortColumn,
        sort_direction: SortDirection,
    ) -> Result<Vec<CardId>> {
        let params = GuiBrowseParams {
            query: query.to_string(),
            reorder_cards: Some(CardsReordering {
                order: sort_direction.into(),
                column_id: column.into(),
            }),
        };

        let ids = self.sender.send::<_, Vec<u64>>("guiBrowse", Some(params))?;
        Ok(ids.into_iter().map(CardId).collect())
    }

    /// Deletes the specified notes
    ///
    /// # Arguments
    ///
    /// * `note_ids` - The IDs of the notes to delete
    pub fn delete_notes(&self, note_ids: &[NoteId]) -> Result<()> {
        let ids: Vec<u64> = note_ids.iter().map(|id| id.0).collect();
        let params = request::DeleteNotesParams { notes: ids };
        self.sender.send::<_, ()>("deleteNotes", Some(params))
    }

    /// Suspends the specified cards
    ///
    /// # Arguments
    ///
    /// * `card_ids` - The IDs of the cards to suspend
    pub fn suspend_cards(&self, card_ids: &[CardId]) -> Result<()> {
        let ids: Vec<u64> = card_ids.iter().map(|id| id.0).collect();
        let params = request::CardIdsParams { cards: ids };
        self.sender.send::<_, ()>("suspend", Some(params))
    }

    /// Unsuspends the specified cards
    ///
    /// # Arguments
    ///
    /// * `card_ids` - The IDs of the cards to unsuspend
    pub fn unsuspend_cards(&self, card_ids: &[CardId]) -> Result<()> {
        let ids: Vec<u64> = card_ids.iter().map(|id| id.0).collect();
        let params = request::CardIdsParams { cards: ids };
        self.sender.send::<_, ()>("unsuspend", Some(params))
    }

    /// Sets the flag color of the specified cards
    ///
    /// # Arguments
    ///
    /// * `card_ids` - The IDs of the cards to flag
    /// * `flag` - The flag color to set (0 = no flag, 1 = red, 2 = orange, etc.)
    pub fn set_flag(&self, card_ids: &[CardId], flag: Flag) -> Result<()> {
        let ids: Vec<u64> = card_ids.iter().map(|id| id.0).collect();
        let params = request::SetFlagParams {
            cards: ids,
            flag: flag as u8,
        };

        self.sender.send::<_, ()>("setFlag", Some(params))
    }

    /// Gets info about the specified note
    ///
    /// # Arguments
    ///
    /// * `note_id` - The ID of the note to get info for
    ///
    /// # Returns
    ///
    /// Detailed information about the note
    pub fn get_note_info(&self, note_id: NoteId) -> Result<request::NoteInfo> {
        let params = request::NoteIdParam { note: note_id.0 };
        self.sender.send("notesInfo", Some(params))
    }

    pub fn find_notes(&self, query: &Query) -> Result<Vec<NoteId>> {
        let params = request::FindNotesParams {
            query: query.to_string(),
        };
        let ids = self.sender.send::<_, Vec<u64>>("findNotes", Some(params))?;
        Ok(ids.into_iter().map(NoteId).collect())
    }

    pub fn update_note_fields(
        &self,
        note_id: NoteId,
        fields: HashMap<String, String>,
    ) -> Result<()> {
        let params = request::UpdateNoteFieldsParams {
            id: note_id.value(),
            fields,
        };

        self.sender
            .send("updateNote", Some(HashMap::from([("note", params)])))
    }

    /// Converts a domain note to a NoteDto for the API
    fn prepare_note_dto(
        &self,
        deck: &Deck,
        note: &Note,
        allow_duplicate: bool,
        duplicate_scope: Option<DuplicateScope>,
    ) -> NoteDto {
        // Prepare media
        let mut audio = Vec::new();
        let mut video = Vec::new();
        let mut picture = Vec::new();

        for field_media in note.media() {
            let media = Media {
                path: field_media.media.source().path().map(|p| p.to_path_buf()),
                url: field_media.media.source().url().map(|u| u.to_string()),
                data: field_media.media.source().data().map(|d| d.to_string()),
                filename: field_media.media.filename().to_string(),
                fields: vec![field_media.field.clone()],
            };

            match field_media.media.media_type() {
                crate::models::MediaType::Audio => audio.push(media),
                crate::models::MediaType::Video => video.push(media),
                crate::models::MediaType::Image => picture.push(media),
            }
        }

        // Configure duplicate handling
        let duplicate_scope_options = if let Some(_scope) = &duplicate_scope {
            // TODO: Not implemented yet
            None
        } else {
            None
        };

        // Create the note DTO
        NoteDto {
            deck_name: deck.name().to_string(),
            model_name: note.model().name().to_string(),
            fields: note.field_values().clone(),
            options: AddNoteOptions {
                allow_duplicate,
                duplicate_scope: duplicate_scope.map(|ds| ds.into()),
                duplicate_scope_options,
            },
            tags: note.tags().iter().cloned().collect(),
            audio,
            video,
            picture,
        }
    }
}

/// Controls how duplicate notes are detected when adding new notes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicateScope {
    /// Check for duplicates only within the specified deck
    Deck,

    /// Check for duplicates across the entire collection
    Collection,
}

impl From<DuplicateScope> for DuplicateScopeDto {
    fn from(value: DuplicateScope) -> Self {
        match value {
            DuplicateScope::Deck => Self::Deck,
            DuplicateScope::Collection => Self::Collection,
        }
    }
}

/// Columns that can be used for sorting in the card browser
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortColumn {
    Answer,
    CardModified,
    Cards,
    Deck,
    Due,
    Ease,
    Lapses,
    Interval,
    NoteCreation,
    NoteMod,
    NoteType,
    OriginalPosition,
    Question,
    Reps,
    SortField,
    Tags,
    Stability,
    Difficulty,
    Retrievability,
}

impl From<SortColumn> for request::ColumnIdentifier {
    fn from(value: SortColumn) -> Self {
        match value {
            SortColumn::Answer => Self::Answer,
            SortColumn::CardModified => Self::CardMod,
            SortColumn::Cards => Self::Cards,
            SortColumn::Deck => Self::Deck,
            SortColumn::Due => Self::Due,
            SortColumn::Ease => Self::Ease,
            SortColumn::Lapses => Self::Lapses,
            SortColumn::Interval => Self::Interval,
            SortColumn::NoteCreation => Self::NoteCreation,
            SortColumn::NoteMod => Self::NoteMod,
            SortColumn::NoteType => Self::Notetype,
            SortColumn::OriginalPosition => Self::OriginalPosition,
            SortColumn::Question => Self::Question,
            SortColumn::Reps => Self::Reps,
            SortColumn::SortField => Self::SortField,
            SortColumn::Tags => Self::Tags,
            SortColumn::Stability => Self::Stability,
            SortColumn::Difficulty => Self::Difficulty,
            SortColumn::Retrievability => Self::Retrievability,
        }
    }
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl From<SortDirection> for request::SortOrder {
    fn from(value: SortDirection) -> Self {
        match value {
            SortDirection::Ascending => Self::Ascending,
            SortDirection::Descending => Self::Descending,
        }
    }
}
