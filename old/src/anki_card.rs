use crate::parameter_types::MediaSource;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeckId(pub u64);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ModelId(pub u64);

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct AnkiDeck {
    id: DeckId,
    name: String,
}

impl AnkiDeck {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id: DeckId(id),
            name,
        }
    }

    pub fn get_id(&self) -> DeckId {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AnkiModelIdentifier {
    id: ModelId,
    name: String,
}

impl AnkiModelIdentifier {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id: ModelId(id),
            name,
        }
    }

    pub fn id(&self) -> ModelId {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Display for AnkiModelIdentifier {
    // Just print the name for now
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Ord for AnkiModelIdentifier {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.0.cmp(&other.id.0)
    }
}

impl PartialOrd for AnkiModelIdentifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AnkiModel {
    id: ModelId,
    name: String,
    // TODO
    // pub fields: Vec<String>,
}

impl AnkiModel {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id: ModelId(id),
            name,
        }
    }

    pub fn get_id(&self) -> ModelId {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct AnkiField {
    model_id: ModelId,
    name: String,
}

impl AnkiField {
    pub fn new(model_id: ModelId, name: String) -> Self {
        Self { model_id, name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub enum MediaType {
    Audio,
    Video,
    Picture,
}

pub struct AnkiCard {
    model: AnkiModelIdentifier,
    /// The fields of the card together with their contents
    fields: HashMap<AnkiField, String>,
    // TODO: Maybe allow multiple fields
    // TODO: Refactor filename. Probably merge with MediaSource,
    //       though serialization might then be difficult
    /// String is the filename
    media: Vec<(MediaType, MediaSource, String, AnkiField)>,
    tags: Vec<String>,
}

impl AnkiCard {
    pub fn get_model_identifier(&self) -> &AnkiModelIdentifier {
        &self.model
    }

    pub fn get_populated_fields(&self) -> impl Iterator<Item = &AnkiField> + '_ {
        self.fields.keys()
    }

    /// Returns the field names of the card together with their contents
    pub fn get_fields_with_content(&self) -> impl Iterator<Item = (&AnkiField, &String)> + '_ {
        self.fields.iter()
        // .map(|(field, content)| (field.name(), &**content))
    }

    pub fn get_tags(&self) -> impl Iterator<Item = &str> + '_ {
        self.tags.iter().map(|tag| &**tag)
    }

    pub fn get_media(
        &self,
    ) -> impl Iterator<Item = &(MediaType, MediaSource, String, AnkiField)> + '_ {
        self.media.iter()
    }
}

#[allow(dead_code)]
pub struct AnkiCardBuilder {
    model: AnkiModelIdentifier,
    fields: HashMap<AnkiField, String>,
    media: Vec<(MediaType, MediaSource, String, AnkiField)>,
    tags: Vec<String>,
}

#[allow(dead_code)]
impl AnkiCardBuilder {
    // TODO: Consider accepting a proper AnkiModel object and check that the fields that are
    //       added are actually part of the model
    pub fn new_for_model(model: AnkiModelIdentifier) -> Self {
        Self {
            model,
            fields: HashMap::new(),
            tags: Vec::new(),
            media: Vec::new(),
        }
    }

    /// Adds `content` to the field `field` as is without escaping
    pub fn add_field_raw(mut self, field: AnkiField, content: &str) -> Self {
        self.fields.insert(field, content.to_string());
        self
    }

    /// Adds the escaped `content` to the field `field`
    pub fn add_field(self, field: AnkiField, content: &str) -> Self {
        let html_escaped_content = html_escape::encode_text(content);
        self.add_field_raw(field, &html_escaped_content)
    }

    pub fn add_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    fn add_media(
        mut self,
        media_type: MediaType,
        source: MediaSource,
        filename: String,
        field: AnkiField,
    ) -> Self {
        self.media.push((media_type, source, filename, field));
        self
    }

    pub fn add_image(self, source: MediaSource, filename: String, field: AnkiField) -> Self {
        self.add_media(MediaType::Picture, source, filename, field)
    }

    pub fn add_audio(self, source: MediaSource, filename: String, field: AnkiField) -> Self {
        self.add_media(MediaType::Audio, source, filename, field)
    }

    pub fn add_video(self, source: MediaSource, filename: String, field: AnkiField) -> Self {
        self.add_media(MediaType::Video, source, filename, field)
    }

    pub fn build(self) -> AnkiCard {
        AnkiCard {
            model: self.model,
            fields: self.fields,
            tags: self.tags,
            media: self.media,
        }
    }
}
