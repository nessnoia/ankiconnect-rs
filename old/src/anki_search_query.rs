use std::collections::HashSet;
use std::marker::PhantomData;

pub struct AnkiSearchQuery(String);

impl AnkiSearchQuery {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct NeedsFieldContent;
pub struct Ready;

#[derive(Debug)]
pub struct AnkiSearchQueryBuilder<State = Ready> {
    parts: Vec<String>,
    negated: bool,
    current_field: Option<String>,
    _state: PhantomData<State>,
}

#[derive(Debug)]
pub enum CardState {
    Due,
    New,
    Learn,
    Review,
    Suspended,
    Buried,
    BuriedSibling,
    BuriedManually,
}

#[derive(Debug)]
pub enum Flag {
    Red = 1,
    Orange = 2,
    Green = 3,
    Blue = 4,
    Pink = 5,
    Turquoise = 6,
    Purple = 7,
}

impl AnkiSearchQueryBuilder<Ready> {
    pub fn new() -> Self {
        Self {
            parts: Vec::new(),
            negated: false,
            current_field: None,
            _state: PhantomData,
        }
    }

    // Methods that can start a query
    pub fn text<S: AsRef<str>>(mut self, text: S) -> AnkiSearchQueryBuilder<Ready> {
        let text = text.as_ref();
        let escaped = Self::escape_special_chars(text);
        self.add_part(escaped);
        self.transition()
    }

    pub fn field<S: AsRef<str>>(mut self, field: S) -> AnkiSearchQueryBuilder<NeedsFieldContent> {
        self.current_field = Some(field.as_ref().to_string());
        self.transition()
    }

    pub fn tag<S: AsRef<str>>(mut self, tag: S) -> AnkiSearchQueryBuilder<Ready> {
        self.add_part(format!("tag:{}", Self::escape_special_chars(tag.as_ref())));
        self.transition()
    }

    pub fn deck<S: AsRef<str>>(mut self, deck: S) -> AnkiSearchQueryBuilder<Ready> {
        let deck = deck.as_ref();
        if deck.contains(' ') {
            self.add_part(format!("deck:\"{}\"", Self::escape_special_chars(deck)));
        } else {
            self.add_part(format!("deck:{}", Self::escape_special_chars(deck)));
        }
        self.transition()
    }

    pub fn card_state(mut self, state: CardState) -> AnkiSearchQueryBuilder<Ready> {
        let state_str = match state {
            CardState::Due => "is:due",
            CardState::New => "is:new",
            CardState::Learn => "is:learn",
            CardState::Review => "is:review",
            CardState::Suspended => "is:suspended",
            CardState::Buried => "is:buried",
            CardState::BuriedSibling => "is:buried-sibling",
            CardState::BuriedManually => "is:buried-manually",
        };
        self.add_part(state_str.to_string());
        self.transition()
    }

    pub fn not(mut self) -> AnkiSearchQueryBuilder<Ready> {
        self.negated = true;
        self.transition()
    }

    pub fn and(self) -> AnkiSearchQueryBuilder<Ready> {
        // Noop, only for readable builder chain
        self
    }

    pub fn or(mut self) -> AnkiSearchQueryBuilder<Ready> {
        self.add_part("or".to_string());
        self.transition()
    }

    pub fn build(self) -> AnkiSearchQuery {
        AnkiSearchQuery(self.parts.join(" "))
    }
}

impl AnkiSearchQueryBuilder<NeedsFieldContent> {
    pub fn with_content<S: AsRef<str>>(mut self, content: S) -> AnkiSearchQueryBuilder<Ready> {
        if let Some(field) = self.current_field.take() {
            let content = content.as_ref();
            let escaped = Self::escape_special_chars(content);
            self.add_part(format!("{}:{}", field, escaped));
        }
        self.transition()
    }
}

impl<State> AnkiSearchQueryBuilder<State> {
    fn add_part(&mut self, part: String) {
        if self.negated {
            self.parts.push(format!("-{}", part));
            self.negated = false;
        } else {
            self.parts.push(part);
        }
    }

    fn escape_special_chars(s: &str) -> String {
        let special_chars: HashSet<char> = ['"', '*', '_', '\\', '(', ')', ':', '-']
            .iter()
            .copied()
            .collect();

        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if special_chars.contains(&c) {
                result.push('\\');
            }
            result.push(c);
        }

        result
    }

    // Helper method to transition between states
    fn transition<NewState>(self) -> AnkiSearchQueryBuilder<NewState> {
        AnkiSearchQueryBuilder {
            parts: self.parts,
            negated: self.negated,
            current_field: self.current_field,
            _state: PhantomData,
        }
    }
}

// Additional implementations for specific properties that can be added in Ready state
impl AnkiSearchQueryBuilder<Ready> {
    pub fn flag(mut self, flag: Flag) -> Self {
        self.add_part(format!("flag:{}", flag as u8));
        self
    }

    pub fn prop_interval_greater_than(mut self, days: u32) -> Self {
        self.add_part(format!("prop:ivl>={}", days));
        self
    }

    pub fn prop_due_in(mut self, days: i32) -> Self {
        self.add_part(format!("prop:due={}", days));
        self
    }

    pub fn prop_reps_less_than(mut self, count: u32) -> Self {
        self.add_part(format!("prop:reps<{}", count));
        self
    }

    pub fn added_in_last_n_days(mut self, days: u32) -> Self {
        self.add_part(format!("added:{}", days));
        self
    }

    pub fn rated_today(mut self) -> Self {
        self.add_part("rated:1".to_string());
        self
    }

    pub fn rated_in_last_n_days(mut self, days: u32) -> Self {
        self.add_part(format!("rated:{}", days));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_text_search() {
        let query = AnkiSearchQueryBuilder::new().text("dog").build();
        assert_eq!(query.as_str(), "dog");
    }

    #[test]
    fn test_field_search() {
        let query = AnkiSearchQueryBuilder::new()
            .field("front")
            .with_content("dog")
            .build();
        assert_eq!(query.as_str(), "front:dog");
    }

    #[test]
    fn test_complex_search() {
        let query = AnkiSearchQueryBuilder::new()
            .field("front")
            .with_content("dog")
            .and()
            .not()
            .tag("marked")
            .build();
        assert_eq!(query.as_str(), "front:dog -tag:marked");
    }

    #[test]
    fn test_deck_with_spaces() {
        let query = AnkiSearchQueryBuilder::new().deck("My Deck").build();
        assert_eq!(query.as_str(), "deck:\"My Deck\"");
    }

    #[test]
    fn test_card_state_1() {
        let query = AnkiSearchQueryBuilder::new()
            .card_state(CardState::Due)
            .and()
            .card_state(CardState::Learn)
            .build();
        assert_eq!(query.as_str(), "is:due is:learn");
    }

    #[test]
    fn test_card_state_2() {
        let query = AnkiSearchQueryBuilder::new()
            .deck("Japanese")
            .field("VocabKanji")
            .with_content("敷衍")
            .and()
            .not()
            .card_state(CardState::Suspended)
            .and()
            .not()
            .card_state(CardState::New)
            .build();
        assert_eq!(
            query.as_str(),
            "deck:Japanese VocabKanji:敷衍 -is:suspended -is:new"
        );
    }
}
