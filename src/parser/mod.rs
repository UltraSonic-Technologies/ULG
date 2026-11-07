//! ULG document parser.
//!
//! This parser is deliberately independent from Pest or another grammar
//! generator. It reads the concrete ULG delimiters directly, which makes
//! the reference syntax unambiguous and avoids fragile whitespace-sensitive
//! rules. The parser accepts arbitrary spaces and line breaks between tokens.

mod cursor;
mod document;
mod error;

pub use document::parse;
pub use error::ParseError;

/// Parses ULG input and returns the language definition.
///
/// The parser validates document structure and literal escapes. Semantic
/// constraints such as duplicate keywords are handled by the validator.
pub fn parse_file(source: &str) -> Result<crate::model::LanguageDefinition, ParseError> {
    parse(source)
}
