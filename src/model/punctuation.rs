//! Punctuation rule representation.
//!
//! The MATCH section stores a name and a pattern. Pattern spelling is
//! preserved exactly as authored. The lexer recognizes conventional paired
//! delimiter names and compiles custom patterns as anchored regular
//! expressions. Unknown names are not assigned special semantics.

/// A named punctuation matcher declared inside a MATCH block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PunctuationDefinition {
    pub name: String,
    pub pattern: String,
}

impl PunctuationDefinition {
    /// Constructs a punctuation definition.
    pub fn new(name: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            pattern: pattern.into(),
        }
    }

    /// Returns true when this rule represents a conventional delimiter pair.
    pub fn is_delimiter_pair(&self) -> bool {
        matches!(
            self.name.to_ascii_uppercase().as_str(),
            "PARENS" | "BRACKETS" | "BRACES"
        )
    }

    /// Returns opening and closing characters for a known pair.
    pub fn delimiter_pair(&self) -> Option<(char, char)> {
        match self.name.to_ascii_uppercase().as_str() {
            "PARENS" => Some(('(', ')')),
            "BRACKETS" => Some(('[', ']')),
            "BRACES" => Some(('{', '}')),
            _ => None,
        }
    }

    /// Returns the pattern exactly as stored in the definition.
    pub fn source_pattern(&self) -> &str {
        &self.pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_parenthesis_pair() {
        let rule = PunctuationDefinition::new("PARENS", r"\(\)");
        assert_eq!(rule.delimiter_pair(), Some(('(', ')')));
    }

    #[test]
    fn custom_names_are_not_assumed_to_be_pairs() {
        let rule = PunctuationDefinition::new("SEMICOLON", ";");
        assert!(!rule.is_delimiter_pair());
        assert_eq!(rule.delimiter_pair(), None);
    }
}
