//! Builder for Anki search queries
//!
//! This module provides a fluent interface for building search queries for Anki.
//! It helps to construct valid search queries with proper escaping and syntax.
//!
//! # Examples
//!
//! ```
//! use ankiconnect_rs::builders::QueryBuilder;
//! use ankiconnect_rs::models::Deck;
//!
//! // Basic query to find cards with the text "biology"
//! let query = QueryBuilder::new().text("biology").build();
//!
//! // More complex query to find cards in a specific deck with certain tags
//! let query = QueryBuilder::new()
//!     .in_deck("Biology::Anatomy")
//!     .and()
//!     .has_tag("important")
//!     .and()
//!     .not()
//!     .has_tag("reviewed")
//!     .build();
//!
//! // Using field-specific search
//! let query = QueryBuilder::new()
//!     .field("Front")
//!     .contains("mitochondria")
//!     .build();
//! ```

use crate::models::{Field, FieldRef};
use crate::Deck;
use std::fmt::{self, Display, Formatter};

/// Represents a complete Anki search query
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    query_string: String,
}

impl Query {
    /// Creates a new query from a string
    ///
    /// This is mostly for internal use. Prefer using `QueryBuilder` to construct queries.
    pub(crate) fn new(query_string: String) -> Self {
        Self { query_string }
    }

    /// Returns the query as a string
    pub fn as_str(&self) -> &str {
        &self.query_string
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.query_string)
    }
}

/// Predefined card states for filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardState {
    /// Cards that are due for review
    Due,
    /// New cards that haven't been studied yet
    New,
    /// Cards currently in the learning phase
    Learning,
    /// Cards in the review phase
    Review,
    /// Cards that have been suspended
    Suspended,
    /// Cards that have been buried
    Buried,
    /// Cards buried because a sibling was answered
    BuriedSibling,
    /// Cards buried manually by the user
    BuriedManual,
}

impl CardState {
    /// Returns the AnkiConnect query string for this state
    fn as_query_str(&self) -> &'static str {
        match self {
            Self::Due => "is:due",
            Self::New => "is:new",
            Self::Learning => "is:learn",
            Self::Review => "is:review",
            Self::Suspended => "is:suspended",
            Self::Buried => "is:buried",
            Self::BuriedSibling => "is:buried-sibling",
            Self::BuriedManual => "is:buried-manually",
        }
    }
}

/// Predefined flag colors for filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    Red = 1,
    Orange = 2,
    Green = 3,
    Blue = 4,
    Pink = 5,
    Turquoise = 6,
    Purple = 7,
}

/// A builder for constructing Anki search queries
///
/// This builder provides a fluent interface for creating properly escaped
/// and formatted Anki search queries. It helps prevent syntax errors and
/// ensures proper escaping of special characters.
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    parts: Vec<String>,
    negated: bool,
    current_field: Option<String>,
}

impl QueryBuilder {
    /// Creates a new, empty query builder
    pub fn new() -> Self {
        Self {
            parts: Vec::new(),
            negated: false,
            current_field: None,
        }
    }

    /// Adds free text to search for across all fields
    ///
    /// Special characters are automatically escaped.
    pub fn text<S: AsRef<str>>(mut self, text: S) -> Self {
        let text = text.as_ref();
        let escaped = Self::escape_special_chars(text);
        self.add_part(escaped);
        self
    }

    /// Specifies a field to search in
    ///
    /// This must be followed by one of the field content methods like `contains`.
    pub fn field<S: AsRef<str>>(mut self, field_name: S) -> FieldQueryBuilder {
        self.current_field = Some(field_name.as_ref().to_string());
        FieldQueryBuilder { builder: self }
    }

    /// Specifies a field to search in using a Field reference
    ///
    /// This must be followed by one of the field content methods like `contains`.
    pub fn in_field<'a>(mut self, field: &Field) -> FieldQueryBuilder {
        self.current_field = Some(field.name().to_string());
        FieldQueryBuilder { builder: self }
    }

    /// Specifies a field to search in using a FieldRef
    ///
    /// This ensures the field actually exists in a model.
    /// This must be followed by one of the field content methods like `contains`.
    pub fn in_field_ref<'a>(mut self, field_ref: FieldRef<'_>) -> FieldQueryBuilder {
        self.current_field = Some(field_ref.name().to_string());
        FieldQueryBuilder { builder: self }
    }

    /// Searches for cards with a specific tag
    pub fn has_tag<S: AsRef<str>>(mut self, tag: S) -> Self {
        self.add_part(format!("tag:{}", Self::escape_special_chars(tag.as_ref())));
        self
    }

    /// Searches for cards in a specific deck
    pub fn in_deck<S: AsRef<str>>(mut self, deck: S) -> Self {
        let deck = deck.as_ref();
        if deck.contains(' ') {
            self.add_part(format!("deck:\"{}\"", Self::escape_special_chars(deck)));
        } else {
            self.add_part(format!("deck:{}", Self::escape_special_chars(deck)));
        }
        self
    }

    /// Searches for cards in the specified deck object
    pub fn in_deck_obj(self, deck: &Deck) -> Self {
        self.in_deck(deck.name())
    }

    /// Searches for cards in a specific card state
    pub fn in_state(mut self, state: CardState) -> Self {
        self.add_part(state.as_query_str().to_string());
        self
    }

    /// Negates the next condition
    pub fn not(mut self) -> Self {
        self.negated = true;
        self
    }

    /// Combines with the next condition using AND (implicit in Anki)
    pub fn and(self) -> Self {
        // This is a no-op in terms of the query string,
        // but helps make the builder more readable
        self
    }

    /// Combines with the next condition using OR
    pub fn or(mut self) -> Self {
        self.add_part("or".to_string());
        self
    }

    /// Searches for cards with a specific flag
    pub fn has_flag(mut self, flag: Flag) -> Self {
        self.add_part(format!("flag:{}", flag as u8));
        self
    }

    /// Searches for cards with an interval greater than or equal to the specified days
    pub fn interval_at_least(mut self, days: u32) -> Self {
        self.add_part(format!("prop:ivl>={}", days));
        self
    }

    /// Searches for cards due in the specified number of days
    pub fn due_in(mut self, days: i32) -> Self {
        self.add_part(format!("prop:due={}", days));
        self
    }

    /// Searches for cards with fewer than the specified number of repetitions
    pub fn reps_less_than(mut self, count: u32) -> Self {
        self.add_part(format!("prop:reps<{}", count));
        self
    }

    /// Searches for cards added within the last n days
    pub fn added_in_last_n_days(mut self, days: u32) -> Self {
        self.add_part(format!("added:{}", days));
        self
    }

    /// Searches for cards rated today
    pub fn rated_today(mut self) -> Self {
        self.add_part("rated:1".to_string());
        self
    }

    /// Searches for cards rated within the last n days
    pub fn rated_in_last_n_days(mut self, days: u32) -> Self {
        self.add_part(format!("rated:{}", days));
        self
    }

    /// Builds the final query
    pub fn build(self) -> Query {
        Query::new(self.parts.join(" "))
    }

    /// Helper method to add a part to the query
    fn add_part(&mut self, part: String) {
        if self.negated {
            self.parts.push(format!("-{}", part));
            self.negated = false;
        } else {
            self.parts.push(part);
        }
    }

    /// Helper method to escape special characters in Anki search
    fn escape_special_chars(s: &str) -> String {
        let needs_escape = |c: char| matches!(c, '"' | '*' | '_' | '\\' | '(' | ')' | ':' | '-');

        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars();

        while let Some(c) = chars.next() {
            if needs_escape(c) {
                result.push('\\');
            }
            result.push(c);
        }

        result
    }
}

/// Helper builder for field-specific queries
///
/// This ensures that field queries are properly structured.
pub struct FieldQueryBuilder {
    builder: QueryBuilder,
}

impl FieldQueryBuilder {
    /// Specifies exact content to match in the field
    pub fn is<S: AsRef<str>>(self, content: S) -> QueryBuilder {
        self.with_content(content)
    }

    /// Specifies content to match in the field
    pub fn contains<S: AsRef<str>>(self, content: S) -> QueryBuilder {
        self.with_content(content)
    }

    /// Internal method to add field content to the query
    fn with_content<S: AsRef<str>>(mut self, content: S) -> QueryBuilder {
        let field_name = self.builder.current_field.take().unwrap();
        let content = content.as_ref();
        let escaped = QueryBuilder::escape_special_chars(content);
        self.builder.add_part(format!("{}:{}", field_name, escaped));
        self.builder
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for common queries
impl QueryBuilder {
    /// Creates a query that searches for cards in the specified deck
    pub fn deck<S: AsRef<str>>(deck: S) -> Self {
        Self::new().in_deck(deck)
    }

    /// Creates a query that searches for cards with the specified tag
    pub fn tag<S: AsRef<str>>(tag: S) -> Self {
        Self::new().has_tag(tag)
    }

    /// Creates a query that searches for cards in the specified state
    pub fn state(state: CardState) -> Self {
        Self::new().in_state(state)
    }

    /// Creates a query that searches for cards with the specified flag
    pub fn flag(flag: Flag) -> Self {
        Self::new().has_flag(flag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_text_search() {
        let query = QueryBuilder::new().text("dog").build();
        assert_eq!(query.as_str(), "dog");
    }

    #[test]
    fn test_field_search() {
        let query = QueryBuilder::new().field("Front").contains("dog").build();
        assert_eq!(query.as_str(), "Front:dog");
    }

    #[test]
    fn test_complex_search() {
        let query = QueryBuilder::new()
            .field("Front")
            .contains("dog")
            .and()
            .not()
            .has_tag("marked")
            .build();
        assert_eq!(query.as_str(), "Front:dog -tag:marked");
    }

    #[test]
    fn test_deck_with_spaces() {
        let query = QueryBuilder::new().in_deck("My Deck").build();
        assert_eq!(query.as_str(), "deck:\"My Deck\"");
    }

    #[test]
    fn test_using_deck_object() {
        let deck = Deck::new(1234, "My Deck".to_string());
        let query = QueryBuilder::new().in_deck_obj(&deck).build();
        assert_eq!(query.as_str(), "deck:\"My Deck\"");
    }

    #[test]
    fn test_card_states() {
        let query = QueryBuilder::new()
            .in_state(CardState::Due)
            .and()
            .in_state(CardState::Learning)
            .build();
        assert_eq!(query.as_str(), "is:due is:learn");
    }

    #[test]
    fn test_special_char_escaping() {
        let query = QueryBuilder::new().text("dog*cat").build();
        assert_eq!(query.as_str(), "dog\\*cat");

        let query = QueryBuilder::new().text("dog (cat)").build();
        assert_eq!(query.as_str(), "dog \\(cat\\)");
    }

    #[test]
    fn test_complex_query_with_or() {
        let query = QueryBuilder::new()
            .in_deck("Japanese")
            .and()
            .field("Vocabulary")
            .contains("敷衍")
            .or()
            .field("Reading")
            .contains("ふえん")
            .and()
            .not()
            .in_state(CardState::Suspended)
            .build();

        assert_eq!(
            query.as_str(),
            "deck:Japanese Vocabulary:敷衍 or Reading:ふえん -is:suspended"
        );
    }

    #[test]
    fn test_convenience_constructors() {
        let query = QueryBuilder::deck("Japanese").build();
        assert_eq!(query.as_str(), "deck:Japanese");

        let query = QueryBuilder::tag("important").build();
        assert_eq!(query.as_str(), "tag:important");

        let query = QueryBuilder::state(CardState::New).build();
        assert_eq!(query.as_str(), "is:new");

        let query = QueryBuilder::flag(Flag::Red).build();
        assert_eq!(query.as_str(), "flag:1");
    }
}
