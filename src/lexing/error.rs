//! Lexical failure diagnostics.

use std::fmt;

use crate::diagnostics::SourceSpan;

/// Error encountered while converting source characters into tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub message: String,
    pub span: SourceSpan,
}

impl LexError {
    /// Creates a lexical error at the beginning of a token or literal.
    pub fn new(message: impl Into<String>, span: SourceSpan) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }

    /// Returns the one-based line and column.
    pub fn location(&self) -> (usize, usize) {
        (self.span.line, self.span.column)
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {}:{}",
            self.message,
            self.span.line,
            self.span.column
        )
    }
}

impl std::error::Error for LexError {}
