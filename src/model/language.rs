//! Complete ULG language definition.
//!
//! The definition is a plain data object. Construction is permissive;
//! callers should invoke semantic validation before trusting a definition
//! for code generation or editor integration. This permits parsers to
//! report multiple problems in a single validation pass.

use super::{CommentDefinition, LanguageVersion, PunctuationDefinition};

/// Parsed metadata and lexical declarations for one language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageDefinition {
    pub name: String,
    pub description: String,
    pub version: LanguageVersion,
    pub keywords: Vec<String>,
    pub comments: Vec<String>,
    pub operators: Vec<String>,
    pub punctuation: Vec<PunctuationDefinition>,
}

impl Default for LanguageDefinition {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            version: LanguageVersion::new(vec![1, 0, 0]).expect("static version"),
            keywords: Vec::new(),
            comments: Vec::new(),
            operators: Vec::new(),
            punctuation: Vec::new(),
        }
    }
}

impl LanguageDefinition {
    /// Creates a definition with the required metadata.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        version: LanguageVersion,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version,
            ..Self::default()
        }
    }

    /// Returns parsed comment rules, reporting malformed declarations.
    pub fn comment_rules(&self) -> Result<Vec<CommentDefinition>, String> {
        self.comments
            .iter()
            .map(|value| CommentDefinition::parse(value))
            .collect()
    }

    /// Returns operators ordered by descending byte length.
    ///
    /// A maximal-munch lexer considers the longest spelling first. This
    /// ensures that equality is recognized before assignment.
    pub fn operators_longest_first(&self) -> Vec<&str> {
        let mut values: Vec<&str> =
            self.operators.iter().map(String::as_str).collect();
        values.sort_by(|left, right| {
            right.len().cmp(&left.len()).then_with(|| left.cmp(right))
        });
        values
    }

    /// Checks whether a word is declared as a keyword.
    pub fn is_keyword(&self, word: &str) -> bool {
        self.keywords.iter().any(|keyword| keyword == word)
    }

    /// Returns a stable count-only overview.
    pub fn summary(&self) -> LanguageSummary {
        LanguageSummary {
            name: self.name.clone(),
            description: self.description.clone(),
            version: self.version.to_string(),
            keyword_count: self.keywords.len(),
            comment_count: self.comments.len(),
            operator_count: self.operators.len(),
            punctuation_count: self.punctuation.len(),
        }
    }
}

/// Count-only overview that does not duplicate lexical declarations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageSummary {
    pub name: String,
    pub description: String,
    pub version: String,
    pub keyword_count: usize,
    pub comment_count: usize,
    pub operator_count: usize,
    pub punctuation_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orders_long_operators_first() {
        let definition = LanguageDefinition {
            operators: vec!["=".into(), "==".into(), "->".into()],
            ..LanguageDefinition::default()
        };
        assert_eq!(
            definition.operators_longest_first(),
            vec!["->", "==", "="]
        );
    }

    #[test]
    fn finds_declared_keyword() {
        let mut definition = LanguageDefinition::default();
        definition.keywords.push("namespace".into());
        assert!(definition.is_keyword("namespace"));
        assert!(!definition.is_keyword("class"));
    }
}
