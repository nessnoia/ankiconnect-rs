use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct AnkiDeck {
    id: u64,
    name: Box<str>,
}

impl AnkiDeck {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name: name.into_boxed_str(),
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AnkiModelIdentifier {
    id: u64,
    name: Box<str>,
}

impl AnkiModelIdentifier {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name: name.into_boxed_str(),
        }
    }

    pub fn get_id(&self) -> u64 {
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
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for AnkiModelIdentifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AnkiModel {
    id: u64,
    name: Box<str>,
    // TODO
    // pub fields: Vec<String>,
}

impl AnkiModel {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name: name.into_boxed_str(),
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct AnkiCard {
    model: AnkiModelIdentifier,
    /// The fields of the cards together with their contents
    /// E.g. "Front" -> "What is the capital of France?"
    fields: HashMap<String, String>,
    tags: Vec<Box<str>>,
}

impl AnkiCard {
    pub fn get_model_identifier(&self) -> &AnkiModelIdentifier {
        &self.model
    }

    pub fn get_fields(&self) -> &HashMap<String, String> {
        &self.fields
    }

    pub fn get_tags(&self) -> impl Iterator<Item = &str> + '_ {
        self.tags.iter().map(|tag| &**tag)
    }
}

#[allow(dead_code)]
pub struct AnkiCardBuilder {
    model: AnkiModelIdentifier,
    fields: HashMap<String, String>,
    tags: Vec<Box<str>>,
}

#[allow(dead_code)]
impl AnkiCardBuilder {
    pub fn new_for_model(model: AnkiModelIdentifier) -> Self {
        Self {
            model,
            fields: HashMap::new(),
            tags: Vec::new(),
        }
    }

    pub fn add_field(mut self, name: &str, content: &str) -> Self {
        self.fields.insert(name.to_string(), content.to_string());
        self
    }

    pub fn add_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string().into_boxed_str());
        self
    }

    pub fn build(self) -> AnkiCard {
        AnkiCard {
            model: self.model,
            fields: self.fields,
            tags: self.tags,
        }
    }
}
