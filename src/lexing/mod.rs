//! Generic tokenizer driven by ULG language definitions.
//!
//! The lexer implements maximal-munch operator recognition, Unicode-aware
//! identifier boundaries, quoted literals, line and block comments, and
//! named punctuation rules. Its behavior is deterministic for a fixed
//! language definition and source file.

mod error;
mod scanner;
mod token;

pub use error::LexError;
pub use token::{Token, TokenKind};

use crate::model::LanguageDefinition;
use scanner::Scanner;

/// Tokenizes source using the declarations in a language definition.
pub fn tokenize(
    definition: &LanguageDefinition,
    source: &str,
) -> Result<Vec<Token>, LexError> {
    Scanner::new(definition, source)?.scan()
}

/// Returns true when a token is a lexical delimiter rather than a word.
pub fn is_delimiter(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Operator | TokenKind::Punctuation | TokenKind::Comment
    )
}
