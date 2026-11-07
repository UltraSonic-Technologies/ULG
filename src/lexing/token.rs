//! Token kinds and source-aware token values.
//!
//! Tokens preserve their original spelling. This allows formatters,
//! diagnostics, syntax highlighters, and transpilers to make decisions without
//! needing to reconstruct source text from semantic values.

use crate::diagnostics::SourceSpan;

/// Category assigned to a lexical token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Keyword,
    Identifier,
    Number,
    String,
    Character,
    Comment,
    Operator,
    Punctuation,
}

/// One lexical token and its exact source range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: SourceSpan,
    pub rule: Option<String>,
}

impl Token {
    /// Constructs a token with a named rule when the category has one.
    pub fn new(
        kind: TokenKind,
        lexeme: impl Into<String>,
        span: SourceSpan,
    ) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            span,
            rule: None,
        }
    }

    /// Attaches the name of a punctuation rule.
    pub fn with_rule(mut self, rule: impl Into<String>) -> Self {
        self.rule = Some(rule.into());
        self
    }

    /// Returns the source line where the token begins.
    pub fn line(&self) -> usize {
        self.span.line
    }

    /// Returns the source column where the token begins.
    pub fn column(&self) -> usize {
        self.span.column
    }

    /// Returns the number of source bytes occupied by the token.
    pub fn byte_len(&self) -> usize {
        self.span.len()
    }

    /// Returns true when the token contains no source text.
    pub fn is_empty(&self) -> bool {
        self.lexeme.is_empty()
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Keyword => "KEYWORD",
            Self::Identifier => "IDENTIFIER",
            Self::Number => "NUMBER",
            Self::String => "STRING",
            Self::Character => "CHARACTER",
            Self::Comment => "COMMENT",
            Self::Operator => "OPERATOR",
            Self::Punctuation => "PUNCTUATION",
        };
        f.write_str(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_uses_stable_labels() {
        assert_eq!(TokenKind::Keyword.to_string(), "KEYWORD");
        assert_eq!(TokenKind::Identifier.to_string(), "IDENTIFIER");
    }

    #[test]
    fn token_exposes_start_position() {
        let span = SourceSpan::new(4, 9, 2, 3);
        let token = Token::new(TokenKind::Identifier, "value", span);
        assert_eq!(token.line(), 2);
        assert_eq!(token.column(), 3);
        assert_eq!(token.byte_len(), 5);
    }
}
