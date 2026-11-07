//! Parser error representation with one-based source positions.

use std::fmt;

/// A syntax error raised while reading a ULG document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl ParseError {
    /// Creates a parser error at a source cursor position.
    pub fn new(
        message: impl Into<String>,
        line: usize,
        column: usize,
        offset: usize,
    ) -> Self {
        Self {
            message: message.into(),
            line,
            column,
            offset,
        }
    }

    /// Returns a short location prefix suitable for terminal diagnostics.
    pub fn location(&self) -> String {
        format!("{}:{}", self.line, self.column)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}: {}",
            self.line,
            self.column,
            self.message
        )
    }
}

impl std::error::Error for ParseError {}
